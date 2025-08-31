use actix_web::{post, HttpResponse, Responder};

#[post("/count")]
async fn count(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/count-single-day")]
async fn count_single_day(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/count-precise")]
async fn count_precise(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}