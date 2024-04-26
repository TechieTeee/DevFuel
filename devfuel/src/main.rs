use std::io::{self, Write};

struct DevFuel {
    tasks: Vec<String>,
    caffeine_tracker: CaffeineTracker,
}

impl DevFuel {
    fn new() -> DevFuel {
        DevFuel {
            // Reserve initial capacity
            tasks: Vec::with_capacity(10), 
            caffeine_tracker: CaffeineTracker::new(),
        }
    }

    fn add_task(&mut self, task: &str) {
        self.tasks.push(task.to_string());
        println!("Task added successfully!");
    }

    fn list_tasks(&self) {
        println!("Tasks:");
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
    }

    fn track_caffeine(&mut self) {
        println!("Enter caffeine intake in mg:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let amount: f64 = input.trim().parse().expect("Invalid input");
        self.caffeine_tracker.log_intake(amount);
        self.caffeine_tracker.display_intake();
        self.caffeine_tracker.recommend_water();
        self.caffeine_tracker.recommend_self_care();
    }

    fn mental_health_check_in(&self) {
        println!("How are you feeling today? (1: Good, 2: Okay, 3: Not so great)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<u32>() {
            Ok(1) => println!("Glad to hear that you're feeling good!"),
            Ok(2) => println!("Hang in there, things will get better!"),
            Ok(3) => {
                println!("Remember, it's okay not to be okay. Here are some suggestions:");
                println!("1. Call a friend or family member to talk.");
                println!("2. Go out with friends for a change of scenery.");
                println!("3. Take a walk outside and enjoy nature.");
                println!("4. Consider seeking professional help if you're feeling really bad.");
            }
            _ => println!("Invalid choice."),
        }
    }
}

struct CaffeineTracker {
    intake: f64,
}

impl CaffeineTracker {
    fn new() -> CaffeineTracker {
        CaffeineTracker { intake: 0.0 }
    }

    fn log_intake(&mut self, amount: f64) {
        self.intake += amount;
    }

    fn display_intake(&self) {
        println!("Total caffeine intake: {:.2} mg", self.intake);
    }

    fn recommend_water(&self) {
        let water_needed = self.intake * 0.75;
        println!("Recommend drinking {:.2} ml of water.", water_needed);
    }

    fn recommend_self_care(&self) {
        if self.intake >= 200.0 {
            println!("High caffeine intake detected. Consider taking a break and engaging in self-care activities like exercise or a short nap.");
            println!("Fun fact: Regular self-care practices can improve productivity and overall well-being!");
        }
    }
}

fn main() {
    println!("Welcome to DevFuel!");

    let mut dev_fuel = DevFuel::new();

    loop {
        println!("Menu:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Track Caffeine");
        println!("4. Mental Health Check-in");
        println!("5. Ask if Touched Grass");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                dev_fuel.add_task(description.trim());
            }
            Ok(2) => dev_fuel.list_tasks(),
            Ok(3) => dev_fuel.track_caffeine(),
            Ok(4) => dev_fuel.mental_health_check_in(),
            Ok(5) => println!("Did you touch grass today?"),
            Ok(6) => break,
            _ => println!("Invalid choice. Please choose again."),
        }
    }

    println!("Thank you for using DevFuel. Have a productive day!");
}
