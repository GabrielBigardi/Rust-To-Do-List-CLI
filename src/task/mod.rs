pub mod operations;

pub struct Task {
    pub id : u32,
    pub name: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, name: String, description: String) -> Task {
        Task {
            id,
            name,
            description,
            completed: false,
        }
    }

    pub fn display(&self) {
        println!("Task {}: {}", self.id, self.name);
        println!("Description: {}", self.description);
        println!("Completed: {}", self.completed);
        println!("");
    }
}