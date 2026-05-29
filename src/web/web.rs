use crate::web::count::{count, count_precise, count_single_day};
use crate::web::dialog::{pick_directory, pick_file, pick_save_file};
use crate::web::log::log_preview;
use crate::web::plot::plot_work_trend;
use crate::web::process::list_processes;
use crate::web::rest::{rest, rest_status, stop_rest};
use actix_files::Files;
use actix_web::{App, HttpServer, rt};
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
                    .service(stop_rest)
                    .service(rest_status)
                    .service(count)
                    .service(count_single_day)
                    .service(count_precise)
                    .service(plot_work_trend)
                    .service(log_preview)
                    .service(pick_directory)
                    .service(pick_file)
                    .service(pick_save_file)
                    .service(list_processes)
                    // Static file server as a fallback for frontend assets
                    .service(Files::new("/", "./frontend").index_file("index.html"))
            })
            .bind(("127.0.0.1", 60606))?
            .run()
            .await
        })
    })
}
