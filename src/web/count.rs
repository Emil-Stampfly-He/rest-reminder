use crate::cli::{parse_datetime_local, parse_datetime_local_day};
use crate::statistic::statistics::{
    TaskWorkSummary, acc_work_time_for_task, acc_work_time_precise_for_task,
    single_day_work_time_for_task, task_work_time_summary,
};
use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
struct CountRequest {
    log_path: String,
    start_time: String,
    end_time: String,
    task: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct CountSingleDayRequest {
    log_path: String,
    date: String,
    task: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct CountPreciseRequest {
    log_path: String,
    start_time: String,
    end_time: String,
    task: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct CountByTaskRequest {
    log_path: String,
    start_time: String,
    end_time: String,
}

#[derive(Serialize)]
struct CountResponse {
    seconds: i64,
}

#[derive(Serialize)]
struct CountByTaskResponse {
    summaries: Vec<TaskWorkSummary>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[post("/count")]
async fn count(req_body: web::Json<CountRequest>) -> impl Responder {
    let log_path = PathBuf::from(&req_body.log_path.as_str());
    let start_time = match parse_datetime_local_day(req_body.start_time.as_str()) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid start time".to_string(),
            });
        }
    };

    let end_time = match parse_datetime_local_day(req_body.end_time.as_str()) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid end time".to_string(),
            });
        }
    };

    if end_time < start_time {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "End time must be greater than start time".to_string(),
        });
    }

    match acc_work_time_for_task(log_path, start_time, end_time, req_body.task.as_deref()) {
        Ok(seconds) => HttpResponse::Ok().json(CountResponse { seconds }),
        Err(_) => HttpResponse::BadRequest().json(ErrorResponse {
            error: "Failed to get work time".to_string(),
        }),
    }
}

#[post("/count-by-task")]
async fn count_by_task(req_body: web::Json<CountByTaskRequest>) -> impl Responder {
    let log_path = PathBuf::from(&req_body.log_path.as_str());
    let start_time = match parse_datetime_local_day(req_body.start_time.as_str()) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid start time".to_string(),
            });
        }
    };

    let end_time = match parse_datetime_local_day(req_body.end_time.as_str()) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid end time".to_string(),
            });
        }
    };

    if end_time < start_time {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "End time must be greater than start time".to_string(),
        });
    }

    match task_work_time_summary(log_path, start_time, end_time) {
        Ok(summaries) => HttpResponse::Ok().json(CountByTaskResponse { summaries }),
        Err(_) => HttpResponse::BadRequest().json(ErrorResponse {
            error: "Failed to get task summary".to_string(),
        }),
    }
}

#[post("/count-single-day")]
async fn count_single_day(req_body: web::Json<CountSingleDayRequest>) -> impl Responder {
    let log_path = PathBuf::from(&req_body.log_path.as_str());
    let date = match parse_datetime_local_day(req_body.date.as_str()) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid date time".to_string(),
            });
        }
    };

    match single_day_work_time_for_task(log_path, date, req_body.task.as_deref()) {
        Ok(seconds) => HttpResponse::Ok().json(CountResponse { seconds }),
        Err(_) => HttpResponse::BadRequest().json(ErrorResponse {
            error: "Failed to get work time".to_string(),
        }),
    }
}

#[post("/count-precise")]
async fn count_precise(req_body: web::Json<CountPreciseRequest>) -> impl Responder {
    let log_path = PathBuf::from(&req_body.log_path.as_str());
    let start_time = match parse_datetime_local(&req_body.start_time.as_str()) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid start time".to_string(),
            });
        }
    };

    let end_time = match parse_datetime_local(&req_body.end_time.as_str()) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid end time".to_string(),
            });
        }
    };

    if end_time < start_time {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "End time must be greater than start time".to_string(),
        });
    }

    match acc_work_time_precise_for_task(log_path, start_time, end_time, req_body.task.as_deref()) {
        Ok(seconds) => HttpResponse::Ok().json(CountResponse { seconds }),
        Err(_) => HttpResponse::BadRequest().json(ErrorResponse {
            error: "Failed to get work time".to_string(),
        }),
    }
}
