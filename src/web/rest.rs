use crate::core::core::run_rest_reminder;
use actix_web::{HttpResponse, Responder, get, post, web};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

static MONITOR_SESSION: LazyLock<Mutex<Option<MonitorSession>>> =
    LazyLock::new(|| Mutex::new(None));

#[derive(Serialize, Deserialize, Clone)]
struct RestRequest {
    log_path: String,
    time: u64,
    app_list: Vec<String>,
}

struct MonitorSession {
    handle: actix_web::rt::task::JoinHandle<()>,
    started_at: DateTime<Local>,
    log_path: String,
    time: u64,
    app_list: Vec<String>,
}

#[derive(Serialize)]
struct RestResponse {
    status: String,
}

#[derive(Serialize)]
struct MonitorStatusResponse {
    running: bool,
    started_at: Option<String>,
    elapsed_seconds: Option<i64>,
    log_path: Option<String>,
    time: Option<u64>,
    app_list: Vec<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn monitor_status_from(session: Option<&MonitorSession>) -> MonitorStatusResponse {
    match session {
        Some(session) => MonitorStatusResponse {
            running: true,
            started_at: Some(session.started_at.to_rfc3339()),
            elapsed_seconds: Some((Local::now() - session.started_at).num_seconds()),
            log_path: Some(session.log_path.clone()),
            time: Some(session.time),
            app_list: session.app_list.clone(),
        },
        None => MonitorStatusResponse {
            running: false,
            started_at: None,
            elapsed_seconds: None,
            log_path: None,
            time: None,
            app_list: Vec::new(),
        },
    }
}

fn clear_finished_session(session: &mut Option<MonitorSession>) {
    if session
        .as_ref()
        .is_some_and(|session| session.handle.is_finished())
    {
        *session = None;
    }
}

#[post("/rest")]
async fn rest(rest_request: web::Json<RestRequest>) -> impl Responder {
    let log_path = PathBuf::from(&rest_request.log_path.as_str());
    let log_path_string = rest_request.log_path.clone();
    let time = rest_request.time;
    let app_list = rest_request.app_list.clone();
    let app_list_for_task = app_list.clone();

    let mut current_session = match MONITOR_SESSION.lock() {
        Ok(session) => session,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to lock monitor session".to_string(),
            });
        }
    };

    clear_finished_session(&mut current_session);
    if current_session.is_some() {
        return HttpResponse::Conflict().json(ErrorResponse {
            error: "Monitoring is already running".to_string(),
        });
    }

    let handle = actix_web::rt::spawn(async move {
        run_rest_reminder(log_path, time, app_list_for_task).await;
    });

    *current_session = Some(MonitorSession {
        handle,
        started_at: Local::now(),
        log_path: log_path_string,
        time,
        app_list,
    });

    HttpResponse::Ok().json(RestResponse {
        status: "started".to_string(),
    })
}

#[post("/rest/stop")]
async fn stop_rest() -> impl Responder {
    let mut current_session = match MONITOR_SESSION.lock() {
        Ok(session) => session,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to lock monitor session".to_string(),
            });
        }
    };

    if let Some(session) = current_session.take() {
        session.handle.abort();
    }

    HttpResponse::Ok().json(RestResponse {
        status: "stopped".to_string(),
    })
}

#[get("/rest/status")]
async fn rest_status() -> impl Responder {
    let mut current_session = match MONITOR_SESSION.lock() {
        Ok(session) => session,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to lock monitor session".to_string(),
            });
        }
    };

    clear_finished_session(&mut current_session);
    HttpResponse::Ok().json(monitor_status_from(current_session.as_ref()))
}
