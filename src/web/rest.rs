use std::path::PathBuf;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::core::core::run_rest_reminder;

#[derive(Serialize, Deserialize, Clone)]
struct RestRequest {
    log_path: String,
    time: u64,
    app_list: Vec<String>,
}

#[post("/rest")]
async fn rest(rest_request: web::Json<RestRequest>) -> impl Responder {
    let log_path = PathBuf::from(&rest_request.log_path.as_str());
    let time = &rest_request.time;
    let app_list = rest_request.clone().app_list;

    run_rest_reminder(log_path, *time, app_list).await;

    HttpResponse::Ok()
}

