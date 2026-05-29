use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Deserialize)]
struct LogPreviewRequest {
    log_path: String,
    limit: Option<usize>,
}

#[derive(Serialize)]
struct LogPreviewResponse {
    entries: Vec<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[post("/log-preview")]
async fn log_preview(req_body: web::Json<LogPreviewRequest>) -> impl Responder {
    let log_path = PathBuf::from(req_body.log_path.as_str());
    let limit = req_body.limit.unwrap_or(8).clamp(1, 50);
    let file = match File::open(log_path) {
        Ok(file) => file,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Failed to open log file".to_string(),
            });
        }
    };

    let mut lines = VecDeque::with_capacity(limit);
    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };

        if line.trim().is_empty() {
            continue;
        }

        if lines.len() == limit {
            lines.pop_front();
        }
        lines.push_back(line);
    }

    HttpResponse::Ok().json(LogPreviewResponse {
        entries: lines.into_iter().collect(),
    })
}
