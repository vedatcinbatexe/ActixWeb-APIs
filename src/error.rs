use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    DuplicateTask { id: u32 },
    LockError,
    IOError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::DuplicateTask { id } => write!(f, "Task with ID {} already exists", id),
            MyError::LockError => write!(f, "Internal error: Mutex lock failed"),
            MyError::IOError(e) => write!(f, "Storage error: {}", e),
        }
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::DuplicateTask { .. } => HttpResponse::Conflict().json(self.to_string()),
            MyError::LockError | MyError::IOError(_) => HttpResponse::InternalServerError().json(self.to_string()),
        }
    }
}