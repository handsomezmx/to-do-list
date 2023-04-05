use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Task {
            description,
            completed: false,
        }
    }
}