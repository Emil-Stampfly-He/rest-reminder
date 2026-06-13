use crate::plugin::plugin::{PLUGIN_ERROR_LOG_PATH, append_plugin_error};
use crate::plugin::template::generate_plugin_template;
use actix_web::{HttpResponse, Responder, get, post, web};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const PLUGIN_DIR: &str = "plugins";

#[derive(Debug, Serialize)]
pub struct PluginInfo {
    pub name: String,
    pub file_name: String,
    pub path: String,
    pub enabled: bool,
    pub version: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub hooks: Vec<String>,
    pub run_in_subprocess: bool,
    pub last_error: Option<String>,
}

#[derive(Serialize)]
struct PluginListResponse {
    plugins: Vec<PluginInfo>,
    errors: Vec<String>,
}

#[derive(Deserialize)]
struct GeneratePluginRequest {
    name: String,
}

#[derive(Serialize)]
struct PluginActionResponse {
    status: String,
    path: Option<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[get("/plugins")]
pub(crate) async fn list_plugins() -> impl Responder {
    match scan_plugins() {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: error.to_string(),
        }),
    }
}

#[post("/plugins/generate")]
pub(crate) async fn generate_plugin(request: web::Json<GeneratePluginRequest>) -> impl Responder {
    let name = request.name.trim();
    if let Err(error) = validate_plugin_name(name) {
        return HttpResponse::BadRequest().json(ErrorResponse { error });
    }

    let path = plugin_path(name);
    if path.exists() {
        return HttpResponse::Conflict().json(ErrorResponse {
            error: "Plugin already exists".to_string(),
        });
    }

    match generate_plugin_template(name).await {
        Ok(path) => HttpResponse::Ok().json(PluginActionResponse {
            status: "generated".to_string(),
            path: Some(path.display().to_string()),
        }),
        Err(error) => {
            append_plugin_error(name, "generate", &error.to_string());
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: error.to_string(),
            })
        }
    }
}

#[post("/plugins/{name}/enable")]
pub(crate) async fn enable_plugin(path: web::Path<String>) -> impl Responder {
    update_plugin_enabled(&path.into_inner(), true)
}

#[post("/plugins/{name}/disable")]
pub(crate) async fn disable_plugin(path: web::Path<String>) -> impl Responder {
    update_plugin_enabled(&path.into_inner(), false)
}

fn update_plugin_enabled(name: &str, enabled: bool) -> HttpResponse {
    if let Err(error) = validate_plugin_name(name) {
        return HttpResponse::BadRequest().json(ErrorResponse { error });
    }

    let path = plugin_path(name);
    if !path.exists() {
        return HttpResponse::NotFound().json(ErrorResponse {
            error: "Plugin not found".to_string(),
        });
    }

    match fs::read_to_string(&path)
        .map(|code| set_ignore_marker(&code, !enabled))
        .and_then(|code| fs::write(&path, code))
    {
        Ok(()) => HttpResponse::Ok().json(PluginActionResponse {
            status: if enabled { "enabled" } else { "disabled" }.to_string(),
            path: Some(path.display().to_string()),
        }),
        Err(error) => {
            append_plugin_error(name, "toggle", &error.to_string());
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: error.to_string(),
            })
        }
    }
}

fn scan_plugins() -> std::io::Result<PluginListResponse> {
    let errors = read_recent_plugin_errors(50);
    if !Path::new(PLUGIN_DIR).exists() {
        return Ok(PluginListResponse {
            plugins: Vec::new(),
            errors,
        });
    }

    let mut plugins = Vec::new();
    for entry in WalkDir::new(PLUGIN_DIR)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "py"))
    {
        let path = entry.path();
        let code = fs::read_to_string(path)?;
        plugins.push(parse_plugin_info(path, &code, &errors));
    }

    plugins.sort_by(|left, right| left.file_name.cmp(&right.file_name));
    Ok(PluginListResponse { plugins, errors })
}

pub fn parse_plugin_info(path: &Path, code: &str, errors: &[String]) -> PluginInfo {
    let file_name = path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string();
    let info_name = plugin_info_value(code, "name").filter(|value| !value.is_empty());
    let last_error = errors
        .iter()
        .rev()
        .find(|line| line.contains(&format!("[{}]", file_name)))
        .cloned();

    PluginInfo {
        name: info_name.unwrap_or_else(|| file_name.clone()),
        file_name,
        path: path.display().to_string(),
        enabled: !should_ignore_plugin(code),
        version: plugin_info_value(code, "version").filter(|value| !value.is_empty()),
        description: plugin_info_value(code, "description").filter(|value| !value.is_empty()),
        author: plugin_info_value(code, "author").filter(|value| !value.is_empty()),
        hooks: plugin_hooks(code),
        run_in_subprocess: run_in_subprocess(code),
        last_error,
    }
}

fn plugin_info_value(code: &str, key: &str) -> Option<String> {
    let pattern = format!(r#"["']{}["']\s*:\s*["']([^"']*)["']"#, regex::escape(key));
    let regex = Regex::new(&pattern).ok()?;
    regex
        .captures(code)
        .and_then(|captures| captures.get(1))
        .map(|value| value.as_str().to_string())
}

fn plugin_hooks(code: &str) -> Vec<String> {
    let regex = Regex::new(r"(?m)^\s*def\s+(on_init|on_work_start|on_break_reminder)\s*\(")
        .expect("hook regex should compile");
    let mut hooks = regex
        .captures_iter(code)
        .filter_map(|captures| captures.get(1).map(|hook| hook.as_str().to_string()))
        .collect::<Vec<_>>();
    hooks.sort();
    hooks.dedup();
    hooks
}

fn should_ignore_plugin(code: &str) -> bool {
    let regex =
        Regex::new(r"(?m)^\s*_SHOULD_IGNORE\s*=\s*1\s*(?:#.*)?$").expect("regex should compile");
    regex.is_match(code)
}

fn run_in_subprocess(code: &str) -> bool {
    let lower_code = code.to_lowercase();
    lower_code.contains("import tkinter")
        || lower_code.contains("from tkinter")
        || lower_code.contains("tkinter.")
        || Regex::new(r"(?m)^\s*_RUN_IN_SUBPROCESS\s*=\s*1\s*(?:#.*)?$")
            .expect("regex should compile")
            .is_match(code)
}

pub fn set_ignore_marker(code: &str, should_ignore: bool) -> String {
    let replacement = if should_ignore {
        "_SHOULD_IGNORE = 1"
    } else {
        "_SHOULD_IGNORE = 0"
    };
    let regex =
        Regex::new(r"(?m)^\s*_SHOULD_IGNORE\s*=\s*[01]\s*(?:#.*)?$").expect("regex should compile");

    if regex.is_match(code) {
        regex.replace(code, replacement).to_string()
    } else {
        format!("{replacement}\n{code}")
    }
}

fn read_recent_plugin_errors(limit: usize) -> Vec<String> {
    let Ok(content) = fs::read_to_string(PLUGIN_ERROR_LOG_PATH) else {
        return Vec::new();
    };

    let mut lines = content
        .lines()
        .rev()
        .take(limit)
        .map(str::to_string)
        .collect::<Vec<_>>();
    lines.reverse();
    lines
}

fn validate_plugin_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Plugin name is required".to_string());
    }

    let regex = Regex::new(r"^[A-Za-z0-9_-]+$").expect("regex should compile");
    if regex.is_match(name) {
        Ok(())
    } else {
        Err("Use only letters, numbers, dashes, and underscores".to_string())
    }
}

fn plugin_path(name: &str) -> PathBuf {
    Path::new(PLUGIN_DIR).join(format!("{name}.py"))
}
