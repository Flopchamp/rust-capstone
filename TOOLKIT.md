# Getting Started with Rust - A Beginner's Guide

**Author:** Harrison Onyango  
**Technology:** Rust Programming Language  
**Project:** Task Manager CLI Application  
**Date:** November 30, 2025

---

## 1. Title & Objective

### What Technology Did I Choose?

**Rust** - A systems programming language focused on safety, speed, and concurrency.

### Why Rust?

- **Industry Growth:** Used by Discord, Microsoft, Amazon, and Dropbox
- **Memory Safety:** Prevents bugs at compile time without garbage collection
- **Performance:** As fast as C/C++
- **Modern Tooling:** Excellent package manager (Cargo) and compiler error messages
- **Career Value:** Increasingly in-demand skill for backend and systems work

### End Goal

Build a working command-line task manager that demonstrates:
- Rust structs and methods
- Ownership and borrowing
- File I/O and persistence
- Interactive user input
- Pattern matching

---

## 2. Quick Summary of the Technology

### What is Rust?

Rust is a systems programming language that guarantees memory safety and thread safety at compile time. Unlike languages with garbage collection (Java, Python), Rust uses an "ownership" system to manage memory automatically.

### Where is it Used?

- **Web Services:** High-performance APIs and servers
- **Command-Line Tools:** Fast, reliable utilities
- **Systems Programming:** Operating systems, device drivers
- **WebAssembly:** Browser-based applications
- **Blockchain:** Cryptocurrency platforms (Solana uses Rust)

### Real-World Example

**Discord** switched their "Read States" service from Go to Rust. This service tracks which messages millions of users have read. The result:
- Latency spikes dropped from seconds to sub-millisecond
- Memory usage decreased significantly
- No more garbage collection pauses

**Source:** Discord Engineering Blog (2020)

---

## 3. System Requirements

### Operating System
- Windows 10/11 (64-bit)
- macOS 10.12+
- Linux (most distributions)

### Required Software

**Windows:**
- Visual Studio Build Tools 2022
- MSVC C++ compiler
- Windows SDK

**All Platforms:**
- Terminal/PowerShell
- Text editor (VS Code recommended)
- Internet connection (for installation)

---

## 4. Installation & Setup Instructions

### Step 1: Install Rust on Windows

1. **Download rustup-init.exe**
   - Go to: https://rustup.rs
   - Click "Download rustup-init.exe"

2. **Run the installer**
   - Double-click `rustup-init.exe`
   - You'll see options about Visual Studio

3. **Install Visual Studio Build Tools**
   - Choose option **1**: "Quick install via the Visual Studio Community installer"
   - Wait 10-15 minutes for download and installation
   - The installer will automatically continue after VS tools are ready

4. **Verify Installation**
   - Close and reopen PowerShell
   - Run these commands:
```powershell
rustc --version
cargo --version
```

**Expected output:**
```
rustc 1.83.0 (90b35a623 2024-11-26)
cargo 1.83.0 (5ffbef321 2024-10-29)
```

### Step 2: Create Your Project
```powershell
# Navigate to Documents
cd $HOME\Documents

# Create project folder
mkdir rust-capstone
cd rust-capstone

# Create Rust project
cargo new task_manager
cd task_manager
```

### Step 3: Test It Works
```powershell
cargo run
```

You should see:
```
   Compiling task_manager v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 2.73s
     Running `target\debug\task_manager.exe`
Hello, world!
```

 **If you see "Hello, world!" - you're ready!**

---

## 5. Minimal Working Example

### The Complete Application

Replace the contents of `src/main.rs` with this code:
```rust
use std::fs;
use std::io::{self, Write};

// Task struct - represents one task
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
    // Create new TaskManager
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

    // Mark task as complete
    fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("✓ Task {} marked as completed!", id);
        } else {
            println!("✗ Task with id {} not found.", id);
        }
    }

    // Save tasks to file
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

    // Load tasks from file
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

    // Load existing tasks
    let _ = manager.load_from_file(filename);

    println!(" Welcome to Rust Task Manager!");
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
```

### Run the Application
```powershell
cargo run
```

### Test It
```
> add Learn Rust ownership
✓ Task added successfully!

> add Build CLI apps
✓ Task added successfully!

> list

=== Your Tasks ===
○ [1] Learn Rust ownership
○ [2] Build CLI apps

> complete 1
✓ Task 1 marked as completed!

> list

=== Your Tasks ===
✓ [1] Learn Rust ownership
○ [2] Build CLI apps

> quit
Tasks saved. Goodbye! 👋
```

---

## 6. AI Prompts Used

### Prompt 1: Understanding Rust Basics

**Prompt:**
```
I'm learning Rust for a capstone project. Explain what makes Rust unique, 
covering: 1) What problems it solves, 2) The ownership system simply, 
3) Why companies like Discord use it, 4) A real-world example.
```

**AI Response Summary:**
The AI explained that Rust solves memory safety issues without garbage collection. The ownership system means each value has one owner, and when that owner goes out of scope, the value is freed. This prevents memory leaks and data races at compile time.

**How It Helped:**
This explanation gave me the mental model needed to understand why Rust's compiler is so strict. Understanding ownership was key to writing the TaskManager methods correctly.

---

### Prompt 2: Interactive Command Loop

**Prompt:**
```
Show me how to create an interactive command-line loop in Rust that reads 
user input, uses pattern matching to handle commands like "add [text]", 
"list", "complete [id]", and "quit".
```

**AI Response Summary:**
The AI showed how to use `loop {}` with `io::stdin().read_line()` and pattern matching with guards (`input if input.starts_with("add ")`). It also explained the need to flush stdout.

**How It Helped:**
I used this structure directly in my main loop. The pattern matching approach made command handling elegant and easy to extend.

---

### Prompt 3: Finding and Modifying in a Vector

**Prompt:**
```
How do I search through a Vec<Task> in Rust to find a task by ID and 
modify its completed field? Show the idiomatic way with iterators.
```

**AI Response Summary:**
The AI demonstrated using `.iter_mut().find()` with a closure, returning `Option<&mut Task>`. Then use `if let Some(task)` to handle the result.

**How It Helped:**
This taught me about mutable iterators and Option handling. I used this exact pattern in the `complete_task()` method.


---

### Prompt 4: File Persistence

**Prompt:**
```
Show me how to save a Vec of structs to a text file in Rust and load 
it back. Use a simple format like pipe-delimited. Include error handling.
```

**AI Response Summary:**
The AI showed using `fs::write()` and `fs::read_to_string()`, formatting data as `id|description|completed`, and returning `io::Result<()>` for error handling.

**How It Helped:**
I implemented this almost exactly as shown. Learning about `Result` and the `?` operator was crucial for understanding Rust's error handling philosophy.


---

### Prompt 5: Ownership in Methods

**Prompt:**
```
Explain the difference between &self and &mut self in Rust method 
signatures. When should I use each? Give examples from a task manager.
```

**AI Response Summary:**
- `&self` = immutable borrow (read-only), use when method doesn't modify state
- `&mut self` = mutable borrow, use when method modifies state
- Example: `list_tasks(&self)` only reads, `add_task(&mut self)` modifies

**How It Helped:**
This clarified my biggest confusion! I now understand why the compiler requires `&mut` for methods that change data.


---

## 7. Common Issues & Fixes

### Issue 1: Missing Visual C++ Prerequisites

**Error:**
```
Rust requires a linker and Windows API libraries but they don't seem to be available.
```

**Solution:**
1. In rustup installer, choose option 1 (Quick install)
2. Wait for Visual Studio Build Tools to install
3. Rustup will continue automatically

**Time to Fix:** 15-20 minutes

---

### Issue 2: Cannot Open kernel32.lib

**Error:**
```
error: linking with `link.exe` failed: exit code: 1181
LINK : fatal error LNK1181: cannot open input file 'kernel32.lib'
```

**Solution:**
1. Open Visual Studio Installer
2. Click "Modify" on Build Tools 2022
3. In "Individual components", check: **MSVC v143 - VS 2022 C++ x64/x86 build tools**
4. Click "Modify" and wait
5. Restart PowerShell
6. Run `cargo run` again

**Time to Fix:** 20 minutes

---

### Issue 3: Borrow Checker Errors

**Error:**
```
cannot borrow `manager` as mutable because it is also borrowed as immutable
```

**Cause:**
Trying to modify data while holding a reference to it.

**Solution:**
Separate operations - don't hold references while modifying:
```rust
// Bad:
let task = &manager.tasks[0];
manager.add_task(...); // Error!

// Good:
manager.add_task(...);
let task = &manager.tasks[0];
```

---

### Issue 4: String vs &str Confusion

**Error:**
```
expected `String`, found `&str`
```

**Solution:**
Use `.to_string()` to convert:
```rust
manager.add_task("Learn Rust".to_string());
```

---

### Issue 5: File Not Found on First Run

**Problem:**
Program crashes when `tasks.txt` doesn't exist.

**Solution:**
Use `let _ =` to ignore the Result:
```rust
let _ = manager.load_from_file(filename);
```





## 8. References

### Essential Documentation
- **The Rust Book:** https://doc.rust-lang.org/book/
- **Rust by Example:** https://doc.rust-lang.org/rust-by-example/
- **Cargo Documentation:** https://doc.rust-lang.org/cargo/

### Installation Help
- **Rustup:** https://rustup.rs/
- **Visual Studio Build Tools:** https://visualstudio.microsoft.com/downloads/

### Community
- **Rust Users Forum:** https://users.rust-lang.org/
- **r/rust:** https://reddit.com/r/rust
- **This Week in Rust:** https://this-week-in-rust.org/

### Video Tutorials
- **Rust Crash Course** (Traversy Media): https://www.youtube.com/watch?v=zF34dRivLOw
- **freeCodeCamp Rust Course:** https://www.youtube.com/watch?v=BpPEoZW5IiY

### Practice
- **Rustlings:** https://github.com/rust-lang/rustlings
- **Exercism Rust Track:** https://exercism.org/tracks/rust

---

## Conclusion

This toolkit demonstrates learning Rust through AI-assisted development. Key takeaways:

**What I Learned:**
- Rust's ownership system prevents memory bugs at compile time
- Pattern matching is powerful for control flow
- The compiler's error messages are genuinely helpful
- File I/O and persistence in systems languages
- Building interactive CLI applications

**Why Rust Matters:**
- Growing demand in industry
- Combines safety with performance
- Excellent tooling and community
- Applicable to many domains

**Time Investment:**
- Installation & setup: 1 hour
- Learning & coding: 2-3 hours
- Documentation: 1-2 hours
- **Total:** 4-6 hours

**Next Steps:**
Build more complex projects, explore web frameworks (Actix, Rocket), or try WebAssembly with Rust.

---

**Project Completed:** November 30, 2025  
**Author:** Harrison Onyango  
**Course:** Moringa School - GenAI for Software Development