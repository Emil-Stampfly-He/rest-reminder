use actix_web::{HttpResponse, Responder, get};
use serde::Serialize;
use std::collections::BTreeSet;
use sysinfo::{ProcessesToUpdate, System};

#[derive(Serialize)]
struct ProcessListResponse {
    processes: Vec<String>,
}

#[get("/processes")]
async fn list_processes() -> impl Responder {
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::All, true);

    let processes = system
        .processes()
        .values()
        .filter_map(|process| {
            let name = process.name().to_string_lossy().trim().to_string();
            (!name.is_empty()).then_some(name)
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    HttpResponse::Ok().json(ProcessListResponse { processes })
}
