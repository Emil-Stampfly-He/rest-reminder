use crate::cli::parse_datetime_local_day;
use crate::statistic::plotter::plot;
use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
struct PlotRequest {
    log_path: String,
    plot_location: String,
    start_date: String,
    end_date: String,
}

#[derive(Serialize)]
struct PlotResponse {
    plot_location: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[post("/plot")]
async fn plot_work_trend(req_body: web::Json<PlotRequest>) -> impl Responder {
    let log_path = PathBuf::from(&req_body.log_path.as_str());
    let plot_location = PathBuf::from(&req_body.plot_location.as_str());
    let start_time = match parse_datetime_local_day(&req_body.start_date.as_str()) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid start date".to_string(),
            });
        }
    };

    let end_time = match parse_datetime_local_day(&req_body.end_date.as_str()) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid end date".to_string(),
            });
        }
    };

    if end_time < start_time {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "End date must be greater than start date".to_string(),
        });
    }

    match plot(log_path, plot_location.clone(), start_time, end_time) {
        Ok(_) => HttpResponse::Ok().json(PlotResponse {
            plot_location: plot_location.to_string_lossy().to_string(),
        }),
        Err(_) => HttpResponse::BadRequest().json(ErrorResponse {
            error: "Failed to plot work trend".to_string(),
        }),
    }
}
