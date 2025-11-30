use std::fs;
use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✓ Task added successfully!");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks yet. Add one to get started!");
            return;
        }

        println!("\n=== Your Tasks ===");
        for task in &self.tasks {
            let status = if task.completed { "✓" } else { "○" };
            println!("{} [{}] {}", status, task.id, task.description);
        }
        println!();
    }

    fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("✓ Task {} marked as completed!", id);
        } else {
            println!("✗ Task with id {} not found.", id);
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut content = String::new();
        for task in &self.tasks {
            content.push_str(&format!(
                "{}|{}|{}\n",
                task.id, task.description, task.completed
            ));
        }
        fs::write(filename, content)?;
        Ok(())
    }

    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let content = fs::read_to_string(filename)?;
        for line in content.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 3 {
                let id: usize = parts[0].parse().unwrap_or(0);
                let description = parts[1].to_string();
                let completed: bool = parts[2].parse().unwrap_or(false);
                
                self.tasks.push(Task {
                    id,
                    description,
                    completed,
                });
                
                if id >= self.next_id {
                    self.next_id = id + 1;
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let mut manager = TaskManager::new();
    let filename = "tasks.txt";

    let _ = manager.load_from_file(filename);

    println!("🦀 Welcome to Rust Task Manager!");
    println!("Commands: add <description>, list, complete <id>, quit");

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match input {
            "quit" | "exit" => {
                manager.save_to_file(filename).unwrap();
                println!("Tasks saved. Goodbye! 👋");
                break;
            }
            "list" => manager.list_tasks(),
            input if input.starts_with("add ") => {
                let description = input[4..].to_string();
                if description.is_empty() {
                    println!("✗ Please provide a task description.");
                } else {
                    manager.add_task(description);
                }
            }
            input if input.starts_with("complete ") => {
                if let Ok(id) = input[9..].parse::<usize>() {
                    manager.complete_task(id);
                } else {
                    println!("✗ Please provide a valid task ID.");
                }
            }
            "" => continue,
            _ => println!("✗ Unknown command. Try: add, list, complete, quit"),
        }
    }
}