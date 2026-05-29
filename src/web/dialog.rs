use actix_web::{HttpResponse, Responder, get};
use serde::Serialize;

#[derive(Serialize)]
struct PathResponse {
    path: Option<String>,
    cancelled: bool,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn path_response(path: Option<String>) -> HttpResponse {
    let cancelled = path.is_none();
    HttpResponse::Ok().json(PathResponse { path, cancelled })
}

fn dialog_error(message: impl Into<String>) -> HttpResponse {
    HttpResponse::InternalServerError().json(ErrorResponse {
        error: message.into(),
    })
}

#[cfg(target_os = "macos")]
fn run_osascript(script: &'static str) -> Result<Option<String>, String> {
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|err| format!("Failed to open picker: {err}"))?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        return Ok((!path.is_empty()).then_some(path));
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.contains("User canceled") || stderr.contains("(-128)") {
        return Ok(None);
    }

    Err(stderr.trim().to_string())
}

#[cfg(target_os = "macos")]
async fn pick_path(script: &'static str) -> HttpResponse {
    match actix_web::rt::task::spawn_blocking(move || run_osascript(script)).await {
        Ok(Ok(path)) => path_response(path),
        Ok(Err(message)) => dialog_error(message),
        Err(err) => dialog_error(format!("Picker task failed: {err}")),
    }
}

#[cfg(not(target_os = "macos"))]
fn rfd_path_response(path: Option<std::path::PathBuf>) -> HttpResponse {
    path_response(path.map(|value| value.to_string_lossy().to_string()))
}

#[get("/dialog/directory")]
async fn pick_directory() -> impl Responder {
    #[cfg(target_os = "macos")]
    {
        pick_path(r#"POSIX path of (choose folder with prompt "Select log directory")"#).await
    }

    #[cfg(not(target_os = "macos"))]
    {
        match actix_web::rt::task::spawn_blocking(|| rfd::FileDialog::new().pick_folder()).await {
            Ok(path) => rfd_path_response(path),
            Err(err) => dialog_error(format!("Picker task failed: {err}")),
        }
    }
}

#[get("/dialog/file")]
async fn pick_file() -> impl Responder {
    #[cfg(target_os = "macos")]
    {
        pick_path(r#"POSIX path of (choose file with prompt "Select focus log file")"#).await
    }

    #[cfg(not(target_os = "macos"))]
    {
        match actix_web::rt::task::spawn_blocking(|| {
            rfd::FileDialog::new()
                .add_filter("Focus log", &["txt", "log"])
                .pick_file()
        })
        .await
        {
            Ok(path) => rfd_path_response(path),
            Err(err) => dialog_error(format!("Picker task failed: {err}")),
        }
    }
}

#[get("/dialog/save-file")]
async fn pick_save_file() -> impl Responder {
    #[cfg(target_os = "macos")]
    {
        pick_path(r#"POSIX path of (choose file name with prompt "Save chart as" default name "plot.png")"#).await
    }

    #[cfg(not(target_os = "macos"))]
    {
        match actix_web::rt::task::spawn_blocking(|| {
            rfd::FileDialog::new()
                .add_filter("PNG image", &["png"])
                .set_file_name("plot.png")
                .save_file()
        })
        .await
        {
            Ok(path) => rfd_path_response(path),
            Err(err) => dialog_error(format!("Picker task failed: {err}")),
        }
    }
}
