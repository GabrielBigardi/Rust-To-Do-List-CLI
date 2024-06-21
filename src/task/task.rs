use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id : String,
    pub name: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(name: String, description: String) -> Task {
        Task {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            completed: false,
        }
    }

    pub fn display(&self) {
        println!("ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Description: {}", self.description);
        println!("Completed: {}", self.completed);
        println!("");
    }
}