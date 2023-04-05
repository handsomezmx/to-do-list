mod task;

use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;
use task::Task;

fn main() {
    let data_path = Path::new("tasks.json");
    let mut tasks = load_tasks(&data_path);

    loop {
        println!("What would you like to do?");
        println!("1: List tasks");
        println!("2: Add task");
        println!("3: Complete task");
        println!("4: Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let option = input.trim().parse::<u32>().unwrap_or(0);

        match option {
            1 => list_tasks(&tasks),
            2 => add_task(&mut tasks),
            3 => complete_task(&mut tasks),
            4 => break,
            _ => println!("Invalid option! Please try again."),
        }

        save_tasks(&tasks, &data_path);
    }
}

fn load_tasks(data_path: &Path) -> Vec<Task> {
    if let Ok(file) = File::open(data_path) {
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &[Task], data_path: &Path) {
    let file = File::create(data_path).expect("Failed to create tasks file");
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &tasks).expect("Failed to write tasks to file");
}

fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks in the list.");
    } else {
        for (index, task) in tasks.iter().enumerate() {
            let status = if task.completed { "✓" } else { " " };
            println!("{} [{}] - {}", index + 1, status, task.description);
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter the task description:");

    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read input");

    let task = Task::new(description.trim().to_string());
    tasks.push(task);

    println!("Task added!");
}

fn complete_task(tasks: &mut Vec<Task>) {
    println!("Enter the task number to mark as completed:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks[index - 1].completed = true;
            println!("Task marked as completed!");
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Invalid input!");
    }
}