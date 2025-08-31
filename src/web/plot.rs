use actix_web::{post, HttpResponse, Responder};

#[post("/plot")]
async fn plot(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}