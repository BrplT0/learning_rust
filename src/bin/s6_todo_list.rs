
#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = vec![Task {
        id: 1,
        description: "hello world".to_string(),
        completed: false
    }];
    println!("{:?}", tasks);
    let task_to_update = &mut tasks[0];
    task_to_update.completed = true;
    println!("{:?}", tasks);
}