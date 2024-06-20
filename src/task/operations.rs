use crate::task::Task;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::io::Result;
use uuid::Uuid;

pub enum Operation {
    Create,
    Read,
    Update,
    Delete,
    Exit,
}

pub fn get_operation() -> Option<Operation> {
    let mut operation = String::new();
    print!("Enter operation (create, read, update, delete, exit): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut operation).unwrap();
    match operation.trim() {
        "create" => Some(Operation::Create),
        "read" => Some(Operation::Read),
        "update" => Some(Operation::Update),
        "delete" => Some(Operation::Delete),
        "exit" => Some(Operation::Exit),
        _ => None,
    }
}

pub fn create_task(tasks: &mut Vec<Task>) {
    let name = read_input("Enter a new task name: ");
    let description = read_input("Enter a description for the task: ");
    let task = Task::new(name, description);
    tasks.push(task);
    println!("Task added successfully!");
    save_tasks(tasks).expect("Failed to save tasks");
}

pub fn read_tasks(tasks: &[Task]) {
    println!("===================");
    println!("List of tasks:");
    println!("===================");
    for task in tasks {
        task.display();
    }
}

pub fn update_task(tasks: &mut Vec<Task>) {
    let mut task_id = String::new();
    print!("Enter the ID of the task to update: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut task_id).unwrap();
    let task_id = task_id.trim();

    if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
        let field = read_input("Enter the field to update (name, description, completed): ");
        match field.trim() {
            "name" => task.name = read_input("Enter the new name for the task: ").trim().to_string(),
            "description" => task.description = read_input("Enter the new description for the task: ").trim().to_string(),
            "completed" => task.completed = read_input("Enter the new completed status for the task (true/false): ")
                .trim()
                .parse()
                .unwrap_or(false),
            "all" => {
                task.name = read_input("Enter the new name for the task: ").trim().to_string();
                task.description = read_input("Enter the new description for the task: ").trim().to_string();
                task.completed = read_input("Enter the new completed status for the task (true/false): ")
                    .trim()
                    .parse()
                    .unwrap_or(false);
            }
            _ => println!("Unknown field"),
        }
        println!("Task updated successfully!");
        save_tasks(tasks).expect("Failed to save tasks");
    } else {
        println!("Task with ID {} not found", task_id);
    }
}

pub fn delete_task(tasks: &mut Vec<Task>) {

    let mut task_id_str = String::new();
    print!("Enter the ID of the task to delete: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut task_id_str).unwrap();

    let task_id = Uuid::parse_str(task_id_str.trim());
    if let Ok(task_id) = task_id {
        tasks.retain(|t| t.id != task_id.to_string());
        println!("Task deleted successfully!");
        save_tasks(tasks).expect("Failed to save tasks");
    } else {
        println!("Invalid task ID format");
    }
}

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn load_tasks() -> Result<Vec<Task>> {
    if !Path::new("tasks.json").exists() {
        return Ok(Vec::new());
    }
    let file = File::open("tasks.json")?;
    let tasks: Vec<Task> = serde_json::from_reader(file)?;
    Ok(tasks)
}

pub fn save_tasks(tasks: &[Task]) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.json")?;
    serde_json::to_writer(file, tasks)?;
    Ok(())
}