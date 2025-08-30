use actix_files::Files;
use actix_web::{get, post, rt, App, HttpResponse, HttpServer, Responder};
use std::thread;

// Start the actix-web server on a dedicated OS thread
pub async fn spawn_web_server() -> thread::JoinHandle<std::io::Result<()>> {
    thread::spawn(|| {
        // Create and run an Actix runtime on this thread.
        rt::System::new().block_on(async move {
            HttpServer::new(|| {
                App::new()
                    // register API routes first so they take precedence over static files
                    .service(rest)
                    .service(count)
                    .service(plot)
                    // static file server as a fallback for frontend assets
                    .service(Files::new("/", "./frontend").index_file("index.html"))
            })
                .bind(("127.0.0.1", 60606))?
                .run()
                .await
        })
    })
}

#[get("/rest")]
async fn rest() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/count")]
async fn count(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/plot")]
async fn plot(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}