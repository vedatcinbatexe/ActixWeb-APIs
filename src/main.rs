use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, ResponseError, error};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: u32,
    description: String,
}

struct AppState {
    app_name: String,
    tasks: Mutex<Vec<Task>>,
}

#[derive(Debug)]
enum MyError {
    DuplicateTask { id: u32 },
    LockError,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::DuplicateTask { id } => write!(f, "Task with ID {} already exists", id),
            MyError::LockError => write!(f, "Internal error: Could not lock database"),
        }
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::DuplicateTask { .. } => HttpResponse::Conflict().json(self.to_string()),
            MyError::LockError => HttpResponse::InternalServerError().json(self.to_string()),
        }
    }
}


#[get("/tasks")]
async fn list_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    web::Json(tasks.to_vec())
}

#[post("/add-task")]
async fn add_task(
    item: web::Json<Task>, 
    data: web::Data<AppState>
) -> Result<impl Responder, MyError> {
    let mut tasks = data.tasks.lock().map_err(|_| MyError::LockError)?;

    if tasks.iter().any(|t| t.id == item.id) {
        return Err(MyError::DuplicateTask { id: item.id });
    }

    tasks.push(item.0);
    Ok(HttpResponse::Created().body("Task added!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        app_name: String::from("Actix-Learn-2026"),
        tasks: Mutex::new(vec![]),
    });

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json("Invalid JSON format!")
                ).into()
            }))
            .service(list_tasks)
            .service(add_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}