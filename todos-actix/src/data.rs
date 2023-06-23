use std::sync::{Arc, Mutex};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub due_date_time: DateTime<Utc>
}

pub struct AppState {
    pub todos: Arc<Mutex<Vec<Todo>>>
}

impl AppState {
    pub fn new() -> AppState {
        Self {
            todos: Arc::new(Mutex::new(Vec::new()))
        }
    }
}
