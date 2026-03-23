use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::{AppState, Task};
use crate::error::MyError;
use std::fs;
use validator::Validate;

const FILE_PATH: &str = "tasks.json";

fn save_to_disk(tasks: &Vec<Task>) -> Result<(), MyError> {
    let data = serde_json::to_string(tasks)
        .map_err(|e| MyError::IOError(e.to_string()))?;
    
    fs::write(FILE_PATH, data)
            .map_err(|e| MyError::IOError(e.to_string()))?;

    Ok(())
}

#[get("/tasks")]
pub async fn list_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    web::Json(tasks.to_vec())
}

#[post("/add-task")]
pub async fn add_task(
    item: web::Json<Task>,
    data: web::Data<AppState>
) -> Result<impl Responder, MyError> {
    item.0.validate().map_err(|e| MyError::ValidationError(e.to_string()))?;

    let mut tasks = data.tasks.lock().map_err(|_| MyError::LockError)?;

    if tasks.iter().any(|t| t.id == item.id) {
        return Err(MyError::DuplicateTask { id: item.id })
    }

    tasks.push(item.0);
    save_to_disk(&tasks)?;

    Ok(HttpResponse::Created().body("Task added and saved!"))
}

#[delete("/tasks/{id}")]
pub async fn delete_task(
    path: web::Path<u32>,
    data: web::Data<AppState>
) -> Result<impl Responder, MyError> {
    let target_id = path.into_inner();

    let mut tasks = data.tasks.lock().map_err(|_| MyError::LockError)?;

    tasks.retain(|task| task.id != target_id);
    save_to_disk(&tasks)?; // PERSISTS

    Ok(HttpResponse::Ok().body("Task deleted and updated on disk"))
}

#[put("/tasks/{id}")]
pub async fn update_task(
    path: web::Path<u32>,
    item: web::Json<Task>,
    data: web::Data<AppState>
) -> Result<impl Responder, MyError> {
    item.0.validate().map_err(|e| MyError::ValidationError(e.to_string()))?;

    let target_id = path.into_inner();
    let mut tasks = data.tasks.lock().map_err(|_| MyError::LockError)?;

    let index = tasks.iter().position(|t| t.id == target_id);

    if let Some(i) = index {
        tasks[i].description = item.description.clone();
        
        save_to_disk(&tasks)?;

        Ok(HttpResponse::Ok().json(tasks[i].clone()))
    } else {
        Ok(HttpResponse::NotFound().body(format!("Task {} not found", target_id)))
    }
}