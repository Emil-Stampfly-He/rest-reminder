use crate::core::core::run_rest_reminder_dynamic;
use actix_web::{HttpResponse, Responder, get, post, web};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use tokio::sync::watch;

static MONITOR_SESSION: LazyLock<Mutex<Option<MonitorSession>>> =
    LazyLock::new(|| Mutex::new(None));

#[derive(Serialize, Deserialize, Clone)]
struct RestRequest {
    log_path: String,
    time: u64,
    app_list: Vec<String>,
    task: Option<String>,
}

struct MonitorSession {
    handle: actix_web::rt::task::JoinHandle<()>,
    started_at: DateTime<Local>,
    log_path: String,
    time: u64,
    app_list: Vec<String>,
    task: Option<String>,
    pause_tx: watch::Sender<bool>,
    app_tx: watch::Sender<Vec<String>>,
    app_started_at: HashMap<String, DateTime<Local>>,
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
    app_statuses: Vec<MonitorAppStatus>,
    task: Option<String>,
    paused: bool,
}

#[derive(Serialize)]
struct MonitorAppStatus {
    name: String,
    elapsed_seconds: i64,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn monitor_status_from(session: Option<&MonitorSession>) -> MonitorStatusResponse {
    match session {
        Some(session) => {
            let now = Local::now();
            MonitorStatusResponse {
                running: true,
                started_at: Some(session.started_at.to_rfc3339()),
                elapsed_seconds: Some((now - session.started_at).num_seconds()),
                log_path: Some(session.log_path.clone()),
                time: Some(session.time),
                app_list: session.app_list.clone(),
                app_statuses: session
                    .app_list
                    .iter()
                    .map(|name| MonitorAppStatus {
                        name: name.clone(),
                        elapsed_seconds: session
                            .app_started_at
                            .get(name)
                            .map(|started_at| (now - *started_at).num_seconds())
                            .unwrap_or(0),
                    })
                    .collect(),
                task: session.task.clone(),
                paused: *session.pause_tx.borrow(),
            }
        }
        None => MonitorStatusResponse {
            running: false,
            started_at: None,
            elapsed_seconds: None,
            log_path: None,
            time: None,
            app_list: Vec::new(),
            app_statuses: Vec::new(),
            task: None,
            paused: false,
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
    let app_list = normalized_apps(&rest_request.app_list);
    let app_list_for_task = app_list.clone();
    let task = rest_request
        .task
        .as_ref()
        .map(|task| task.trim().to_string())
        .filter(|task| !task.is_empty());
    let task_for_task = task.clone();

    let mut current_session = match MONITOR_SESSION.lock() {
        Ok(session) => session,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to lock monitor session".to_string(),
            });
        }
    };

    clear_finished_session(&mut current_session);
    if let Some(session) = current_session.as_mut() {
        session.log_path = log_path_string;
        session.time = time;
        session.task = task;
        session.app_list = app_list.clone();
        update_app_started_at(&mut session.app_started_at, &app_list);

        if session.app_tx.send(app_list).is_err() {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to update monitored apps".to_string(),
            });
        }

        return HttpResponse::Ok().json(RestResponse {
            status: "updated".to_string(),
        });
    }

    let (pause_tx, pause_rx) = watch::channel(false);
    let (app_tx, app_rx) = watch::channel(app_list.clone());
    let app_started_at = app_list
        .iter()
        .map(|app| (app.clone(), Local::now()))
        .collect();

    let handle = actix_web::rt::spawn(async move {
        run_rest_reminder_dynamic(
            log_path,
            time,
            app_list_for_task,
            task_for_task,
            Some(pause_rx),
            app_rx,
        )
        .await;
    });

    *current_session = Some(MonitorSession {
        handle,
        started_at: Local::now(),
        log_path: log_path_string,
        time,
        app_list,
        task,
        pause_tx,
        app_tx,
        app_started_at,
    });

    HttpResponse::Ok().json(RestResponse {
        status: "started".to_string(),
    })
}

fn normalized_apps(apps: &[String]) -> Vec<String> {
    apps.iter().fold(Vec::new(), |mut normalized, app| {
        let app = app.trim();
        if !app.is_empty()
            && !normalized
                .iter()
                .any(|item: &String| item.eq_ignore_ascii_case(app))
        {
            normalized.push(app.to_string());
        }
        normalized
    })
}

fn update_app_started_at(
    app_started_at: &mut HashMap<String, DateTime<Local>>,
    app_list: &[String],
) {
    let now = Local::now();
    app_started_at.retain(|name, _| app_list.iter().any(|app| app == name));
    for app in app_list {
        app_started_at.entry(app.clone()).or_insert(now);
    }
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

#[post("/rest/pause")]
async fn pause_rest() -> impl Responder {
    set_pause_state(true, "paused")
}

#[post("/rest/resume")]
async fn resume_rest() -> impl Responder {
    set_pause_state(false, "resumed")
}

fn set_pause_state(paused: bool, status: &str) -> HttpResponse {
    let mut current_session = match MONITOR_SESSION.lock() {
        Ok(session) => session,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to lock monitor session".to_string(),
            });
        }
    };

    clear_finished_session(&mut current_session);
    let Some(session) = current_session.as_ref() else {
        return HttpResponse::Conflict().json(ErrorResponse {
            error: "Monitoring is not running".to_string(),
        });
    };

    if session.pause_tx.send(paused).is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to update monitor pause state".to_string(),
        });
    }

    HttpResponse::Ok().json(RestResponse {
        status: status.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, http::StatusCode, test};
    use std::sync::LazyLock;
    use tokio::sync::Mutex as AsyncMutex;

    static TEST_LOCK: LazyLock<AsyncMutex<()>> = LazyLock::new(|| AsyncMutex::new(()));

    fn reset_monitor_session() {
        let mut current_session = MONITOR_SESSION
            .lock()
            .expect("monitor session lock should not be poisoned");
        if let Some(session) = current_session.take() {
            session.handle.abort();
        }
    }

    fn body_to_string(body: actix_web::web::Bytes) -> String {
        String::from_utf8(body.to_vec()).expect("response body should be utf-8")
    }

    #[actix_web::test]
    async fn status_reports_stopped_when_no_monitor_is_running() {
        let _guard = TEST_LOCK.lock().await;
        reset_monitor_session();

        let app = test::init_service(App::new().service(rest_status)).await;
        let req = test::TestRequest::get().uri("/rest/status").to_request();
        let response = test::call_service(&app, req).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = body_to_string(test::read_body(response).await);
        assert!(body.contains(r#""running":false"#));
        assert!(body.contains(r#""app_list":[]"#));
        assert!(body.contains(r#""paused":false"#));
    }

    #[actix_web::test]
    async fn stop_is_idempotent_when_no_monitor_is_running() {
        let _guard = TEST_LOCK.lock().await;
        reset_monitor_session();

        let app = test::init_service(App::new().service(stop_rest)).await;
        let req = test::TestRequest::post().uri("/rest/stop").to_request();
        let response = test::call_service(&app, req).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body = body_to_string(test::read_body(response).await);
        assert!(body.contains(r#""status":"stopped"#));
    }

    #[actix_web::test]
    async fn starting_monitor_sets_status_and_updates_running_apps() {
        let _guard = TEST_LOCK.lock().await;
        reset_monitor_session();

        let app = test::init_service(
            App::new()
                .service(rest)
                .service(pause_rest)
                .service(resume_rest)
                .service(rest_status)
                .service(stop_rest),
        )
        .await;
        let request = RestRequest {
            log_path: std::env::temp_dir()
                .join("rest_reminder_route_test")
                .to_string_lossy()
                .to_string(),
            time: 3600,
            app_list: vec!["rest-reminder-test-process-that-should-not-exist".to_string()],
            task: Some("test".to_string()),
        };

        let start_req = test::TestRequest::post()
            .uri("/rest")
            .set_json(&request)
            .to_request();
        let start_response = test::call_service(&app, start_req).await;
        assert_eq!(start_response.status(), StatusCode::OK);

        let status_req = test::TestRequest::get().uri("/rest/status").to_request();
        let status_response = test::call_service(&app, status_req).await;
        assert_eq!(status_response.status(), StatusCode::OK);
        let status_body = body_to_string(test::read_body(status_response).await);
        assert!(status_body.contains(r#""running":true"#));
        assert!(status_body.contains(r#""paused":false"#));
        assert!(status_body.contains("rest-reminder-test-process-that-should-not-exist"));

        let pause_req = test::TestRequest::post().uri("/rest/pause").to_request();
        let pause_response = test::call_service(&app, pause_req).await;
        assert_eq!(pause_response.status(), StatusCode::OK);

        let paused_status_req = test::TestRequest::get().uri("/rest/status").to_request();
        let paused_status_response = test::call_service(&app, paused_status_req).await;
        let paused_status_body = body_to_string(test::read_body(paused_status_response).await);
        assert!(paused_status_body.contains(r#""paused":true"#));

        let resume_req = test::TestRequest::post().uri("/rest/resume").to_request();
        let resume_response = test::call_service(&app, resume_req).await;
        assert_eq!(resume_response.status(), StatusCode::OK);

        let resumed_status_req = test::TestRequest::get().uri("/rest/status").to_request();
        let resumed_status_response = test::call_service(&app, resumed_status_req).await;
        let resumed_status_body = body_to_string(test::read_body(resumed_status_response).await);
        assert!(resumed_status_body.contains(r#""paused":false"#));

        let update_request = RestRequest {
            app_list: vec![
                "rest-reminder-test-process-that-should-not-exist".to_string(),
                "rest-reminder-second-test-process".to_string(),
            ],
            ..request.clone()
        };
        let update_req = test::TestRequest::post()
            .uri("/rest")
            .set_json(&update_request)
            .to_request();
        let update_response = test::call_service(&app, update_req).await;
        assert_eq!(update_response.status(), StatusCode::OK);
        let update_body = body_to_string(test::read_body(update_response).await);
        assert!(update_body.contains(r#""status":"updated"#));

        let updated_status_req = test::TestRequest::get().uri("/rest/status").to_request();
        let updated_status_response = test::call_service(&app, updated_status_req).await;
        let updated_status_body = body_to_string(test::read_body(updated_status_response).await);
        assert!(updated_status_body.contains(r#""running":true"#));
        assert!(updated_status_body.contains("rest-reminder-second-test-process"));

        let stop_req = test::TestRequest::post().uri("/rest/stop").to_request();
        let stop_response = test::call_service(&app, stop_req).await;
        assert_eq!(stop_response.status(), StatusCode::OK);
        reset_monitor_session();
    }

    #[actix_web::test]
    async fn pause_and_resume_reject_when_no_monitor_is_running() {
        let _guard = TEST_LOCK.lock().await;
        reset_monitor_session();

        let app = test::init_service(App::new().service(pause_rest).service(resume_rest)).await;

        let pause_req = test::TestRequest::post().uri("/rest/pause").to_request();
        let pause_response = test::call_service(&app, pause_req).await;
        assert_eq!(pause_response.status(), StatusCode::CONFLICT);

        let resume_req = test::TestRequest::post().uri("/rest/resume").to_request();
        let resume_response = test::call_service(&app, resume_req).await;
        assert_eq!(resume_response.status(), StatusCode::CONFLICT);
    }
}
