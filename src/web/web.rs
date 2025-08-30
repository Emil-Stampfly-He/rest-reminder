use actix_files::Files;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use std::thread;

// Start the actix-web server on a dedicated OS thread
pub async fn spawn_web_server() -> thread::JoinHandle<std::io::Result<()>> {
    thread::spawn(|| {
        // Create and run an Actix runtime on this thread.
        actix_web::rt::System::new().block_on(async move {
            HttpServer::new(|| {
                App::new()
                    .service(Files::new("/", "./frontend").index_file("index.html"))
                    .service(hello)
                    .service(echo)
            })
                .bind(("127.0.0.1", 60606))?
                .run()
                .await
        })
    })
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}