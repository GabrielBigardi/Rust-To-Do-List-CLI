use crate::task::Task;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;

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
    let task = Task::new(tasks.len() as u32 + 1, name, description);
    tasks.push(task);
    println!("Task added successfully!");
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
    let task_id: u32 = read_input("Enter the ID of the task to update: ")
        .trim()
        .parse()
        .unwrap();

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
    } else {
        println!("Task with ID {} not found", task_id);
    }
}

pub fn delete_task(tasks: &mut Vec<Task>) {
    let task_id: u32 = read_input("Enter the ID of the task to delete: ")
        .trim()
        .parse()
        .unwrap();
    tasks.retain(|t| t.id != task_id);
    println!("Task deleted successfully!");
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}