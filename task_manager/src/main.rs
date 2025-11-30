fn main() {
    println!("Hello, world!");
}
use std::io::{self, Write};

// Task struct - represents a single task
#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

// TaskManager - manages all tasks
struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    // Create a new TaskManager
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // Add a new task
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

    // List all tasks
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
}

fn main() {
    println!("🦀 Welcome to Rust Task Manager!");
    
    let mut manager = TaskManager::new();
    
    // Test by adding some tasks
    manager.add_task("Learn Rust basics".to_string());
    manager.add_task("Build task manager".to_string());
    manager.list_tasks();
    
    println!("Goodbye! 👋");
}