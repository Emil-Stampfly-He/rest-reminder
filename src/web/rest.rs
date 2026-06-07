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
    async fn starting_monitor_sets_status_and_rejects_duplicate_start() {
        let _guard = TEST_LOCK.lock().await;
        reset_monitor_session();

        let app = test::init_service(
            App::new()
                .service(rest)
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
        assert!(status_body.contains("rest-reminder-test-process-that-should-not-exist"));

        let duplicate_req = test::TestRequest::post()
            .uri("/rest")
            .set_json(&request)
            .to_request();
        let duplicate_response = test::call_service(&app, duplicate_req).await;
        assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);

        let stop_req = test::TestRequest::post().uri("/rest/stop").to_request();
        let stop_response = test::call_service(&app, stop_req).await;
        assert_eq!(stop_response.status(), StatusCode::OK);
        reset_monitor_session();
    }
}
