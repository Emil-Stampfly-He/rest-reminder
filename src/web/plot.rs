use crate::cli::parse_datetime_local_day;
use crate::statistic::plotter::plot;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
struct PlotRequest {
    log_path: String,
    plot_location: String,
    start_date: String,
    end_date: String,
}

#[post("/plot")]
async fn plot_work_trend(req_body: web::Json<PlotRequest>) -> impl Responder {
    let log_path = PathBuf::from(&req_body.log_path.as_str());
    let plot_location = PathBuf::from(&req_body.plot_location.as_str());
    let start_time = match parse_datetime_local_day(&req_body.start_date.as_str()) {
        Ok(t) => t,
        Err(_) => return HttpResponse::BadRequest().body("Invalid start date"),
    };
    
    let end_time = match parse_datetime_local_day(&req_body.end_date.as_str()) {
        Ok(t) => t,
        Err(_) => return HttpResponse::BadRequest().body("Invalid end date"),
    };

    match plot(log_path, plot_location, start_time, end_time) {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(_) => HttpResponse::BadRequest().body("Failed to plot work trend"),
    }
}