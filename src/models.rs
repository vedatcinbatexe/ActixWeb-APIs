use serde::{Serialize, Deserialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
}

pub struct AppState {
    pub tasks: Mutex<Vec<Task>>,
}