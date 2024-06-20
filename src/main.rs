mod task;

use task::Task;
use task::operations::{self, Operation};

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        match operations::get_operation() {
            Some(Operation::Create) => operations::create_task(&mut tasks),
            Some(Operation::Read) => operations::read_tasks(&tasks),
            Some(Operation::Update) => operations::update_task(&mut tasks),
            Some(Operation::Delete) => operations::delete_task(&mut tasks),
            Some(Operation::Exit) => {
                println!("Exiting...");
                break;
            },
            None => println!("Unknown operation"),
        }
    }
}