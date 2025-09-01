use crate::web::count::count;
use crate::web::rest::rest;
use crate::web::plot::plot_work_trend;
use actix_files::Files;
use actix_web::{rt, App, HttpServer};
use std::thread;

// Start the actix-web server on a dedicated OS thread
pub async fn spawn_web_server() -> thread::JoinHandle<std::io::Result<()>> {
    thread::spawn(|| {
        // Create and run an Actix runtime on this thread.
        rt::System::new().block_on(async move {
            HttpServer::new(|| {
                App::new()
                    // Register API routes first so they take precedence over static files
                    .service(rest)
                    .service(count)
                    .service(plot_work_trend)
                    // Static file server as a fallback for frontend assets
                    .service(Files::new("/", "./frontend").index_file("index.html"))
            })
                .bind(("127.0.0.1", 60606))?
                .run()
                .await
        })
    })
}