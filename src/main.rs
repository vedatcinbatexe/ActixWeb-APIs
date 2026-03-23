use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

// 1. Define our Data Model
#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    description: String,
}

// 2. Define our Shared State (In-memory list of tasks)
struct AppState {
    app_name: String,
    tasks: Mutex<Vec<Task>>, // Mutex allows safe cross-thread updates
}

// 3. GET Handler: Uses Path Extractor
#[get("/info/{user}")]
async fn get_info(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let user = path.into_inner();
    format!("Hello {user}, welcome to {}!", data.app_name)
}

// 4. GET Handler: Returning tasks list
#[get("/tasks")]
async fn list_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();

    web::Json(tasks.to_vec())
}

// 5. POST Handler: Uses JSON Extractor
#[post("/add-task")]
async fn add_task(item: web::Json<Task>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    tasks.push(item.0); // item.0 gets the inner Task from the web::Json wrapper
    HttpResponse::Ok().body("Task added!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize our shared state
    let app_data = web::Data::new(AppState {
        app_name: String::from("Actix-Learn-2026"),
        tasks: Mutex::new(vec![]),
    });

    println!("Learning server started at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone()) // Pass state to the app
            .service(get_info)
            .service(add_task)
            .service(list_tasks)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}