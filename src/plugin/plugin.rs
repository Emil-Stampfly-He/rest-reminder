use std::ffi::CString;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};
use std::path::{Path, PathBuf};
use std::process::Command;
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
    path: PathBuf,
    // If true, the plugin will always be executed in a subprocess. This is
    // used for plugins that use GUI toolkits like tkinter which require the
    // main thread on many platforms (Windows) and will raise
    // "RuntimeError: main thread is not in main loop" if called from a
    // background thread.
    force_subprocess: bool,
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

            // Heuristic: if the plugin source imports or references tkinter,
            // mark it to be run in a subprocess to avoid calling tkinter from
            // a non-main thread which causes the runtime error on Windows.
            let lower_code = code.to_lowercase();
            let force_subprocess = lower_code.contains("import tkinter")
                || lower_code.contains("from tkinter")
                || lower_code.contains("tkinter.");

            let plugin_script = PluginScript {
                name: plugin_name.clone(),
                module: module.unbind(),
                path: path.to_path_buf(),
                force_subprocess,
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

        // We'll call plugin hooks without blocking the caller:
        // - If a plugin sets `_RUN_IN_SUBPROCESS = 1` (in its module), we spawn
        //   an external Python process to run the hook (good for GUI toolkits like tkinter).
        // - Otherwise, we invoke the hook in a detached thread which acquires the GIL,
        //   so the current thread is not blocked by long-running plugin code.
        for plugin in &self.activated_plugins {
            let plugin_name = plugin.name.clone();
            let plugin_path = plugin.path.clone();
            let hook = hook_name.to_string();
            let ctx = context.clone();

            // Check `_RUN_IN_SUBPROCESS` flag in the plugin module or the
            // load-time heuristic `force_subprocess` (e.g., detects tkinter).
            let run_in_subprocess = if plugin.force_subprocess {
                true
            } else {
                Python::with_gil(|py| {
                    let module = plugin.module.as_ref();
                    match module.getattr(py, "_RUN_IN_SUBPROCESS") {
                        Ok(val) => val.extract::<i32>(py).unwrap_or(0) == 1,
                        Err(_) => false,
                    }
                })
            };

            if run_in_subprocess {
                // Spawn an external python process to run the hook. This keeps GUI
                // toolkits and blocking UI code in a separate process so they won't
                // block the main application or other plugins.
                let python_code = format!(
                    "import runpy\nmod = runpy.run_path(r'{}')\nif '{}' in mod:\n    try:\n        mod['{}']({{}})\n    except Exception as e:\n        import sys, traceback; traceback.print_exc(file=sys.stderr)",
                    plugin_path.display(), hook, hook
                );

                match Command::new("python").arg("-c").arg(python_code).spawn() {
                    Ok(_child) => {
                        println!("  {} {} {}", "✓".bright_green(),
                                 plugin_name.bright_cyan(),
                                 format!("spawned {} in subprocess", hook).white());
                    }
                    Err(e) => {
                        println!("  {} {} {} - {}", "✗".bright_red(),
                                 plugin_name.bright_cyan(),
                                 format!("failed to spawn {}", hook).white(),
                                 e.to_string().red());
                    }
                }
                continue;
            }

            // Otherwise, run the hook in a detached thread that will acquire the GIL
            // locally so we don't block the caller.
            tokio::spawn(async move {
                Python::with_gil(|py| {
                    let py_context = PyDict::new(py);
                    py_context.set_item("message", &ctx.message).unwrap();
                    py_context.set_item("timestamp", &ctx.timestamp).unwrap();
                    py_context.set_item("work_duration", ctx.work_duration).unwrap();

                    if let Ok(module) = PyModule::import(py, &plugin_name) {
                        if let Ok(hook_func) = module.getattr(hook.as_str()) {
                            match hook_func.call((py_context.clone(),), None) {
                                Ok(_) => {
                                    println!("  {} {} {}", "✓".bright_green(),
                                             plugin_name.bright_cyan(),
                                             format!("executed {}", hook).white());
                                }
                                Err(e) => {
                                    println!("  {} {} {} - {}", "✗".bright_red(),
                                             plugin_name.bright_cyan(),
                                             format!("failed {}", hook).white(),
                                             e.to_string().red());
                                }
                            }
                        } else {
                            println!("  {} {} {}", "○".bright_black(),
                                     plugin_name.bright_cyan(),
                                     format!("no {} hook", hook).bright_black());
                        }
                    }
                });
            });
        }

        Ok(())
    }
    
    // Check if plugins shouldn't be loaded when initializing
    fn should_ignore_plugin(code: &str) -> bool {
        // Self::should_ignore_plugin_regex(code)
        Self::should_ignore_plugin_python_exec(code)
    }

    // Match _SHOULD_IGNORE = 1 by regex
    #[allow(unused)]
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

    // Allow checking _SHOULD_IGNORE in real Python env, safer but slower
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

#[derive(Clone)]
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
