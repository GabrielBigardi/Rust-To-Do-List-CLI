mod task;

use task::operations::{self, Operation};

fn main() {
    let mut tasks = operations::load_tasks().expect("Failed to load tasks");

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