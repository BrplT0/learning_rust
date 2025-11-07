use std::io;

#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

fn add_task(task_list:&mut Vec<Task>, task: Task) {
    task_list.push(task);
}

fn remove_task(task_list:&mut Vec<Task>, id:u32) {
    if let Some(index) = task_list.iter().position(|t| t.id == id) {
        task_list.remove(index);
        println!("Task with ID {} removed successfully.", id);
    } else {
        println!("Error: Task with ID {} not found.", id);
    }
}

fn complete_task(task_list: &mut Vec<Task>, id: u32) {
        if let Some(index) = task_list.iter().position(|t| t.id == id) {
            task_list[index].completed = true;
            println!("Task with ID {} updated successfully.", id);
        } else {
            println!("Error: Task with ID {} not found.", id);
        }
}

fn main() {
    let mut task_list: Vec<Task> = vec![];
    println!("Welcome to ToDo App!:\n");
    let mut i = 0;

    loop {
        let mut operation = String::new();
        println!("Select the operation:\nadd_task(A)\nremove_task(R)\ncomplete_task(C)\nprint(P)\nstop(S)\n");
        io::stdin().read_line(&mut operation).expect("Failed to read line");

        match operation.trim().to_uppercase().as_str() {
            "A" => {
                println!("Enter a Task to add");
                let mut task: String = String::new();
                io::stdin().read_line(&mut task).expect("Failed to read line");
                task = task.trim().to_string();
                add_task(&mut task_list, Task { id: i, description: task, completed: false });
                i = i + 1
            }
            "R" => {
                println!("Enter the ID of the task to remove");
                let mut id: String = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                remove_task(&mut task_list, id.trim().parse().unwrap_or_else(|_| 0));
            }
            "C" => {
                println!("Enter the ID of the task to complete");
                let mut id: String = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                complete_task(&mut task_list, id.trim().parse().unwrap_or_else(|_| 0));
            }
            "P" => {
                println!("{:?}", task_list);
            }
            "S" => {
                break;
            }
            _ => println!("Invalid operation\n")
        }
    }
}
