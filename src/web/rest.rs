use crate::core::core::run_rest_reminder;
use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
struct RestRequest {
    log_path: String,
    time: u64,
    app_list: Vec<String>,
}

#[derive(Serialize)]
struct RestResponse {
    status: String,
}

#[post("/rest")]
async fn rest(rest_request: web::Json<RestRequest>) -> impl Responder {
    let log_path = PathBuf::from(&rest_request.log_path.as_str());
    let time = rest_request.time;
    let app_list = rest_request.clone().app_list;

    actix_web::rt::spawn(async move {
        run_rest_reminder(log_path, time, app_list).await;
    });

    HttpResponse::Ok().json(RestResponse {
        status: "started".to_string(),
    })
}
