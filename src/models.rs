use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct Task {
    pub id: u32,

    #[validate(length(min = 10, max = 100, message = "Description must be between 10 and 100 characters"))]
    pub description: String,
}

pub struct AppState {
    pub tasks: Mutex<Vec<Task>>,
}