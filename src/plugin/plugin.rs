use std::ffi::CString;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};
use std::path::Path;
use chrono::Local;
use walkdir::WalkDir;
use colored::*;
use pyo3::exceptions::PyIOError;
use regex::Regex;

const IGNORE_PATTERN: &str = r"^\s*_SHOULD_IGNORE\s*=\s*1\s*$";

pub struct PluginManager {
    inactivated_plugins: Vec<PluginScript>,
    activated_plugins: Vec<PluginScript>,
}

struct PluginScript {
    name: String,
    module: Py<PyModule>,
}

impl PluginManager {
    pub fn new() -> PyResult<Self> {
        Ok(PluginManager {
            inactivated_plugins: Vec::new(),
            activated_plugins: Vec::new(),
        })
    }

    // Load all Python plugins in specified directory
    pub fn load_plugins(&mut self, plugin_dir: &str) -> PyResult<()> {
        if !Path::new(plugin_dir).exists() {
            println!("{} {}", "Plugin directory not found:".yellow(), plugin_dir.red());
            return Ok(());
        }

        println!("{} {}", "Loading plugins from:".bright_green().bold(), plugin_dir.cyan());

        // Scan every .py file
        for entry in WalkDir::new(plugin_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "py"))
        {
            let path = entry.path();
            match self.load_plugin(path) {
                Ok(plugin_name) => {
                    println!("  {} {}", "✓ Loaded plugin:".bright_green(), plugin_name.bright_cyan());
                }
                Err(e) => {
                    println!("  {} {} - {}", "✗ Failed to load:".bright_red(), 
                           path.display(), e.to_string().red());
                }
            }
        }

        println!("{} {} {} {} {}", "Loaded".bright_green().bold(),
                 self.activated_plugins.len().to_string().bright_yellow(),
                 "plugin(s) successfully.".bright_green().bold(),
                 self.inactivated_plugins.len().to_string().yellow(),
                 "plugin(s) ignored.".green()
        );
        Ok(())
    }

    // Load single plugin
    fn load_plugin(&mut self, path: &Path) -> PyResult<String> {
        let plugin_name = path.file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Read Python script
        let code = std::fs::read_to_string(path)
            .map_err(|e| PyErr::new::<PyIOError, _>(format!("Failed to read file: {}", e)))?;

        Python::with_gil(|py| {
            let module = PyModule::from_code(
                py,
                CString::new(&*code).unwrap().as_c_str(),
                CString::new(path.to_str().unwrap_or(&plugin_name)).unwrap().as_c_str(),
                CString::new(&*plugin_name).unwrap().as_c_str(),
            )?;

            let plugin_script = PluginScript {
                name: plugin_name.clone(),
                module: module.unbind(),
            };

            if Self::should_ignore_plugin(&code) {
                self.inactivated_plugins.push(plugin_script);
            } else {
                self.activated_plugins.push(plugin_script);
            }

            Ok(plugin_name)
        })
    }

    // Trigger hooks
    pub fn trigger_hook(&self, hook_name: &str, context: &PluginContext) -> PyResult<()> {
        if self.activated_plugins.is_empty() {
            return Ok(());
        }

        println!("{} {} {}", "Triggering hook:".bright_magenta().bold(), 
               hook_name.bright_yellow(), 
               format!("for {} plugin(s)", self.activated_plugins.len()).bright_magenta());

        Python::with_gil(|py| {
            // Use of plugin context is optional
            // Plugin context is a Python dict
            let py_context = PyDict::new(py);
            py_context.set_item("message", &context.message)?;
            py_context.set_item("timestamp", &context.timestamp)?;
            py_context.set_item("work_duration", context.work_duration)?;

            // Call hooks
            for plugin in &self.activated_plugins {
                let module = plugin.module.as_ref();
                
                // Examine if plugin has specified hooks
                // (Reflectionally) call Python function
                if let Ok(hook_func) = module.getattr(py, hook_name) {
                    // py_context is the only param in every hook
                    match hook_func.call(py, (py_context.clone(),), None) {
                        Ok(_) => {
                            println!("  {} {} {}", "✓".bright_green(), 
                                   plugin.name.bright_cyan(), 
                                   format!("executed {}", hook_name).white());
                        }
                        Err(e) => {
                            println!("  {} {} {} - {}", "✗".bright_red(), 
                                   plugin.name.bright_cyan(), 
                                   format!("failed {}", hook_name).white(),
                                   e.to_string().red());
                        }
                    }
                } else {
                    println!("  {} {} {}", "○".bright_black(), 
                           plugin.name.bright_cyan(), 
                           format!("no {} hook", hook_name).bright_black());
                }
            }
            Ok(())
        })
    }
    
    // Check if plugins shouldn't be loaded when initializing
    fn should_ignore_plugin(code: &str) -> bool {
        Self::should_ignore_plugin_regex(code)
        // Self::should_ignore_plugin_python_exec(code)
    }

    // Match _SHOULD_IGNORE = 1 by regex
    fn should_ignore_plugin_regex(code: &str) -> bool {
        let regex = Regex::new(IGNORE_PATTERN).unwrap();

        for line in code.lines() {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
                continue;
            }

            if regex.is_match(trimmed_line) {
                return true;
            }
        }
        false
    }

    /// Allow checking _SHOULD_IGNORE in real Python env, safer but slower
    #[allow(unused)]
    fn should_ignore_plugin_python_exec(code: &str) -> bool {
        Python::with_gil(|py| {
            // Set a temporary variable to detect _SHOULD_IGNORE
            let check_code = format!("{}\ntry:\n    _result = _SHOULD_IGNORE == 1\nexcept NameError:\n    _result = False", code);
            let check_code_cstr = CString::new(check_code).unwrap();

            match py.run(check_code_cstr.as_c_str(), None, None) {
                Ok(()) => {
                    // Evaluate _result
                    let eval_code = CString::new("_result").unwrap();
                    match py.eval(eval_code.as_c_str(), None, None) {
                        Ok(result) => {
                            result.extract::<bool>().unwrap_or_else(|_| false)
                        }
                        Err(_) => false,
                    }
                }
                Err(_) => {
                    // Failed to detect, don't ignore
                    false
                }
            }
        })
    }

    pub fn plugin_count(&self) -> usize {
        self.inactivated_plugins.len() + self.activated_plugins.len()
    }
    
    pub fn list_plugins(&self) {
        if self.inactivated_plugins.is_empty() && self.activated_plugins.is_empty() {
            println!("{}", "No plugins loaded".yellow());
            return;
        }

        let mut all_plugins = vec![];
        self.inactivated_plugins.iter().for_each(|plugin| {
            all_plugins.push(plugin);
        });
        self.activated_plugins.iter().for_each(|plugin| {
            all_plugins.push(plugin);
        });

        println!("{}", "Loaded plugins:".bright_green().bold());
        for (index, plugin) in all_plugins.into_iter().enumerate() {
            println!("  {}. {}", (index + 1).to_string().bright_yellow(), 
                   plugin.name.bright_cyan());
        }
    }
}

pub struct PluginContext {
    pub message: String,
    pub timestamp: String,
    pub work_duration: u64,
}

impl PluginContext {
    pub fn new(message: &str, work_duration: u64) -> Self {
        PluginContext {
            message: message.to_string(),
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            work_duration,
        }
    }
}
