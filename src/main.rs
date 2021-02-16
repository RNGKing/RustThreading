use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
mod workerstate;


use workerstate::WorkerState;

struct AppState{
    work_state: Mutex<WorkerState>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("You are connected to the server")
}

#[get("/start")]
async fn start_work(data: web::Data<AppState>) -> impl Responder {
    match data.work_state.lock().unwrap().StartWork(){
        Ok(()) => HttpResponse::Ok().body("Success starting work thread"),
        Err(err) => HttpResponse::Ok().body(err),
    }
}

#[get("/end")]
async fn end_work(data: web::Data<AppState>) -> impl Responder {
    match  data.work_state.lock().unwrap().EndWork(){
        Ok(()) => HttpResponse::Ok().body("Successfully ended the thread"),
        Err(err) => HttpResponse::Ok().body(err),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {work_state: Mutex::new(WorkerState::new())})
            .service(hello)
            .service(start_work)
            .service(end_work)
    }).bind("127.0.0.1:8080")?
    .run()
    .await
}
