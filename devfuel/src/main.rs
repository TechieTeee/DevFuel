use std::io;

fn main() {
    println!("Welcome to DevFuel!");

    let mut tasks = Vec::new();

    loop {
        println!("Menu:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => add_task(&mut tasks),
            Ok(2) => list_tasks(&tasks),
            Ok(3) => break,
            _ => println!("Invalid choice. Please choose again."),
        }
    }

    println!("Thank you for using DevFuel Tracker. Have a productive day!");
}

fn add_task(tasks: &mut Vec<String>) {
    println!("Enter task description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description)
        .expect("Failed to read line");
    tasks.push(description.trim().to_string());
    println!("Task added successfully!");
}

fn list_tasks(tasks: &Vec<String>) {
    println!("Tasks:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{}. {}", index + 1, task);
    }
}
