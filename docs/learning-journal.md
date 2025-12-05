# Rust Task Manager - Learning Journal

**Author:** Harrison Onyango Aloo
**Project:** Moringa School Capstone - Rust Task Manager  


---

##  Project Overview

**Technology Chosen:** Rust Programming Language

**Project Type:** Command-Line Task Manager Application

**Why I Chose Rust:**
Rust is a systems programming language focused on safety, speed, and concurrency. It's increasingly used in production at companies like Discord, Microsoft, and Amazon. I wanted to learn a language that offers both performance and memory safety without a garbage collector.

**End Goal:**
Build a fully functional CLI task manager that can:
- Add tasks with unique IDs
- List all tasks with status indicators
- Mark tasks as complete
- Persist data to a file between sessions
- Handle user input interactively

---

##  Day 1: Installation, Setup & First Working Code

**Date:** November 30, 2025  
**Time Investment:** ~2.5 hours  
**Status:**  SUCCESSFUL - Working code deployed!

---

## Installation Journey

### Initial Installation Attempt
 
**Tool Used:** rustup-init.exe from https://rustup.rs

**First Challenge Encountered:**
```
warn: It looks like you have an existing rustup settings file at:
warn: C:\Users\alooh\.rustup\settings.toml

Rust Visual C++ prerequisites
Rust requires a linker and Windows API libraries but they don't seem to be available.
```

**What This Meant:**
Rust on Windows requires Microsoft Visual C++ build tools to compile code. Unlike interpreted languages (Python, JavaScript), Rust compiles to native machine code and needs:
1. A C++ compiler (MSVC)
2. A linker (link.exe)
3. Windows SDK libraries

**Solution Applied:**
Selected option 1: "Quick install via the Visual Studio Community installer"

**Result:** 
Visual Studio Build Tools installer launched and began downloading components.

**Time:** ~15 minutes (including download time)

---

### Second Challenge: Missing MSVC Compiler

**Problem Encountered:**

After Visual Studio Build Tools installed, attempted first compilation:
```powershell
PS C:\Users\alooh\Documents\rust-capstone\task_manager> cargo run
   Compiling task_manager v0.1.0
error: linking with `link.exe` failed: exit code: 1181
  = note: LINK : fatal error LNK1181: cannot open input file 'kernel32.lib'
```

**Root Cause Analysis:**

The error indicated the linker couldn't find Windows system libraries. This meant:
-  Rust was installed correctly
-  Windows SDKs were installed
-  MSVC C++ compiler was NOT installed

**Diagnosis Process:**

1. Checked Windows SDK installation:
```powershell
dir "C:\Program Files (x86)\Windows Kits\10\Lib"
# Result:  SDK found
```

2. Opened Visual Studio Installer and checked components
3. Discovered in "Individual components" tab:
   -  Windows 11 SDK (10.0.26100.6901) - Checked
   -  Windows 10 SDK (10.0.19041.0) - Checked
   -  MSVC v143 - VS 2022 C++ x64/x86 build tools - **NOT CHECKED**

**Solution:**

1. Opened Visual Studio Installer
2. Clicked "Modify" on Visual Studio Build Tools 2022
3. Navigated to "Individual components" tab
4. Checked: **MSVC v143 - VS 2022 C++ x64/x86 build tools (Latest)**
5. Clicked "Modify" button
6. Installer prompted to close VS processes - clicked "Continue"
7. Waited for MSVC installation (~10 minutes)
8. Closed all PowerShell windows
9. Opened fresh PowerShell window
10. Navigated back to project and ran `cargo run`

**Time to Resolve:** ~25 minutes (including installation)

---

###  SUCCESS - First Rust Program Compiled!



**Command Executed:**
```powershell
PS C:\Users\alooh\Documents\rust-capstone\task_manager> cargo run
   Compiling task_manager v0.1.0 (C:\Users\alooh\Documents\rust-capstone\task_manager)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.73s
     Running `target\debug\task_manager.exe`
Hello, world!
```

**What This Confirmed:**
-  Rust compiler (rustc) working
-  Cargo build system working
-  MSVC linker working
-  Complete toolchain operational

**Compilation Speed:** 2.73 seconds (impressively fast!)

---

## Installation Verification Details

**Rust Version:**
```powershell
PS> rustc --version
rustc 1.91.1 (ed61e7d7e 2025-11-07)
```

**Cargo Version:**
```powershell
PS> cargo --version
cargo 1.91.1 (ea2d97820 2025-10-29)
```

**Visual Studio Components Installed:**
- Visual Studio Build Tools 2022
- MSVC v143 - VS 2022 C++ x64/x86 build tools (Latest)
- Windows 11 SDK (10.0.26100.6901)
- Windows 10 SDK (10.0.19041.0)
- C++ core desktop features
- C++ AddressSanitizer
- C++ CMake tools for Windows

---

## Project Structure Created

**Location:** `C:\Users\alooh\Documents\rust-capstone\`
```
rust-capstone/
├── docs/
│   └── learning-journal.md (this file)
└── task_manager/
    ├── .gitignore
    ├── Cargo.toml
    └── src/
        └── main.rs
```

**Cargo.toml Contents:**
```toml
[package]
name = "task_manager"
version = "0.1.0"
edition = "2021"

[dependencies]
```

---

## First Working Application - Task Manager v0.1

### Implementation

**Code Structure:**
```rust
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
    fn new() -> Self { ... }
    fn add_task(&mut self, description: String) { ... }
    fn list_tasks(&self) { ... }
}

fn main() {
    // Test implementation
}
```

### Test Run Output
```powershell
PS C:\Users\alooh\Documents\rust-capstone\task_manager> cargo run
   Compiling task_manager v0.1.0 (C:\Users\alooh\Documents\rust-capstone\task_manager)
warning: unused imports: `Write` and `self`
 --> src\main.rs:1:15
  |
1 | use std::io::{self, Write};
  |               ^^^^  ^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `task_manager` (bin "task_manager") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
     Running `target\debug\task_manager.exe`
🦀 Welcome to Rust Task Manager!
✓ Task added successfully!
✓ Task added successfully!

=== Your Tasks ===
○ [1] Learn Rust basics
○ [2] Build task manager

Goodbye! 👋
```

**Compilation Time:** 0.76 seconds ⚡

**Note on Warning:** 
The unused imports warning is expected - we'll need `std::io::{self, Write}` tomorrow when implementing the interactive input loop.

---

## Features Implemented -

###  Task Struct
- Stores task ID (auto-incrementing number)
- Stores description (user's task text)
- Stores completion status (true/false)
- Uses `#[derive(Debug)]` for debugging output

###  TaskManager Struct
- Maintains vector of all tasks
- Tracks next available ID
- Provides clean interface for task operations

###  Add Task Functionality
```rust
fn add_task(&mut self, description: String)
```
- Creates new task with auto-incrementing ID
- Adds to task vector
- Displays confirmation message
- Uses `&mut self` because it modifies the manager's state

###  List Tasks Functionality
```rust
fn list_tasks(&self)
```
- Displays all tasks with formatting
- Shows completion status with symbols (○ incomplete, ✓ complete)
- Shows task ID and description
- Handles empty task list gracefully
- Uses `&self` because it only reads data

---

## Key Rust Concepts Learned

### 1. Structs - Custom Data Types
Structs let us create custom types that group related data:
```rust
struct Task {
    id: usize,        // Unsigned integer (always positive)
    description: String,  // Owned string data
    completed: bool,     // Boolean flag
}
```

### 2. Ownership & Borrowing
**The Core Principle:** Each value has exactly one owner at a time.

**Borrowing Rules:**
- `&self` - Immutable borrow (read-only access)
- `&mut self` - Mutable borrow (can modify data)
- Can have many immutable borrows OR one mutable borrow (not both)

**In Practice:**
```rust
fn list_tasks(&self)  // Only reads → immutable borrow
fn add_task(&mut self)  // Modifies → mutable borrow
```

### 3. Vec<T> - Dynamic Arrays
```rust
tasks: Vec<Task>
```
- Growable array stored on the heap
- Type-safe (can only hold Task objects)
- Automatically manages memory

### 4. String vs &str
- `String` - Owned, heap-allocated, mutable
- `&str` - Borrowed string slice, read-only
- Use `.to_string()` to convert &str → String

**Example:**
```rust
fn add_task(&mut self, description: String)  // Takes ownership
manager.add_task("Learn Rust".to_string())   // Create owned String
```

### 5. Pattern Matching with if/let
```rust
let status = if task.completed { "✓" } else { "○" };
```
Rust's if is an expression that returns a value.

### 6. Iterators
```rust
for task in &self.tasks {  // Borrow each task
    println!("{}", task.description);
}
```
The `&` is crucial - without it, we'd move the tasks out of the vector.

---

## AI Prompts Used 

### Prompt 1: Understanding Basic Rust Concepts

**Date:** November 30, 2025  


**Prompt:**
```
I just built my first Rust program - a task manager that can add and 
list tasks. Explain these Rust concepts I used in simple terms:

1. What does &self vs &mut self mean in method signatures?
2. Why do I use .to_string() when adding a task?
3. What's the difference between Vec::new() and vec![]?
4. How does the ownership system prevent bugs?

Keep it beginner-friendly with practical examples.
```

**AI Response Summary:**
[To be filled in after running the prompt]

**Key Takeaways:**
[What I learned from this prompt]



**How It Helped My Project:**
[Specific ways this knowledge improved my understanding or code]

---

## Common Errors & Solutions

### Error 1: Missing Visual C++ Prerequisites

**Error Message:**
```
Rust requires a linker and Windows API libraries but they don't seem to be available.
```

**Cause:**
Rust needs Microsoft Visual C++ build tools on Windows to compile code.

**Solution:**
1. In rustup installer, choose option 1 (Quick install)
2. Let Visual Studio Build Tools install automatically
3. Wait for completion (~15 minutes)

**Prevention:**
Always install Visual Studio Build Tools before Rust on Windows.

**Documentation Reference:** https://rust-lang.github.io/rustup/installation/windows.html

---

### Error 2: Linker Error - Cannot Open kernel32.lib

**Error Message:**
```
error: linking with `link.exe` failed: exit code: 1181
LINK : fatal error LNK1181: cannot open input file 'kernel32.lib'
```

**Cause:**
Windows SDK was installed but MSVC C++ compiler was not checked during Visual Studio Build Tools installation.

**Solution Steps:**
1. Open Visual Studio Installer
2. Click "Modify" on Build Tools 2022
3. Go to "Individual components" tab
4. Check: "MSVC v143 - VS 2022 C++ x64/x86 build tools"
5. Click "Modify" and wait for installation
6. Close and reopen PowerShell
7. Run `cargo run` again

**Why This Happens:**
The Windows SDK provides the libraries (kernel32.lib) but the MSVC compiler provides the linker that accesses them. Both are required.

**Time to Fix:** 20-25 minutes

**Documentation Reference:** 
- https://stackoverflow.com/questions/tagged/rust+windows+linker
- https://doc.rust-lang.org/book/ch01-01-installation.html

---

### Error 3: Unused Imports Warning

**Warning Message:**
```
warning: unused imports: `Write` and `self`
 --> src\main.rs:1:15
  |
1 | use std::io::{self, Write};
```

**Cause:**
We imported `std::io::{self, Write}` for future use but haven't used them yet.

**Is This a Problem?**
No - it's just a warning. The code compiles and runs fine.

**When Will We Use Them?**
Tomorrow (Day 2) when we add interactive user input:
- `std::io::self` - for reading user input
- `Write` - for flushing output buffer

**Solution (if you want to remove the warning now):**
Temporarily comment out line 1:
```rust
// use std::io::{self, Write};
```

We'll uncomment it tomorrow when needed.

---

## Technical Challenges Overcome

### Challenge 1: Understanding Rust's Compilation Process

**Issue:** 
Coming from interpreted languages, the compilation step was new.

**Learning:**
- Rust compiles to native machine code (not bytecode)
- This requires a linker and system libraries
- Compilation catches many bugs before runtime
- Trade-off: Slower development cycle but safer, faster code

### Challenge 2: Grasping Ownership Concepts

**Issue:**
The concept of "borrowing" was initially confusing.

**Mental Model That Helped:**
Think of data like a library book:
- Only one person can OWN it (write in it)
- Many people can BORROW it (read it)
- Can't modify while someone else is reading
- Compiler enforces these rules at compile time

**Practical Example:**
```rust
fn list_tasks(&self)       // Borrow for reading
fn add_task(&mut self)     // Borrow for writing
```

---

## Resources Used

### Official Documentation
- **The Rust Book:** https://doc.rust-lang.org/book/
  - Read: Chapter 1 (Getting Started)
  - Skimmed: Chapter 4 (Ownership)
  
- **Rust by Example:** https://doc.rust-lang.org/rust-by-example/
  - Useful for syntax reference

- **Cargo Book:** https://doc.rust-lang.org/cargo/
  - Understanding `Cargo.toml` and project structure

### Installation & Setup
- **Rustup:** https://rustup.rs/
- **Visual Studio Build Tools:** https://visualstudio.microsoft.com/downloads/

### Troubleshooting
- **Rust Users Forum:** https://users.rust-lang.org/
- **Stack Overflow - Rust Tag:** https://stackoverflow.com/questions/tagged/rust

### Tools Considered
- **VS Code** with rust-analyzer extension (recommended for Day 2)
- **Visual Studio 2022** (too heavy for this project)
- **RustRover** by JetBrains (paid, more advanced)

---

## Reflections 

### What Went Well 
- Successfully navigated two major installation issues
- Understood and fixed linker problems independently
- Wrote working Rust code on first day
- Grasped basic ownership concepts
- Project structure is clean and organized

### What Was Challenging 
- Windows-specific toolchain setup (MSVC requirements)
- Understanding why certain components were needed
- Differentiating between String and &str
- Remembering when to use & in method parameters

### Surprises 
- Rust's error messages are incredibly helpful
- Compilation is much faster than expected
- The ownership system makes sense with practice
- Cargo automates so much (project structure, builds, dependencies)

### Key Insights 
1. **Front-load the hard stuff:** Installation issues are common but solvable
2. **Document everything:** Future me will thank present me
3. **Compiler is a teacher:** Error messages guide you to correct code
4. **Rust's safety is real:** Can't accidentally create memory bugs
5. **Community matters:** Good documentation and tools make learning easier

---

## Time Breakdown - Day 1

| Activity | Duration | Notes |
|----------|----------|-------|
| Initial Rust installation attempt | 10 mins | Hit first error |
| VS Build Tools installation | 20 mins | Automatic download/install |
| Troubleshooting linker errors | 30 mins | Diagnosis and research |
| MSVC compiler installation | 15 mins | Additional component |
| First successful cargo run | 5 mins | "Hello, world!" |
| Learning Rust basics | 20 mins | Reading docs, understanding concepts |
| Implementing Task struct | 15 mins | Writing and testing |
| Implementing TaskManager | 20 mins | Methods and logic |
| Testing and debugging | 10 mins | Running cargo run multiple times |
| Documentation | 25 mins | This journal |
| **Total Day 1** | **~2.5 hours** | Productive session! |

---


### Goals 

**Feature 1: Interactive Command Loop**
- Implement infinite loop for user commands
- Read user input from stdin
- Parse commands (add, list, complete, quit)
- Use pattern matching for command dispatch

**Feature 2: Complete Task Functionality**
```rust
fn complete_task(&mut self, id: usize) {
    // Find task by ID
    // Mark as completed
    // Handle case where ID doesn't exist
}
```

**Feature 3: File Persistence**
```rust
fn save_to_file(&self, filename: &str) -> io::Result<()>
fn load_from_file(&mut self, filename: &str) -> io::Result<()>
```
- Save tasks to `tasks.txt`
- Load tasks on startup
- Handle file I/O errors gracefully

### AI Prompts to Use 

**Prompt 1: Command Loop**
```
Show me how to create an interactive command-line loop in Rust that:
- Reads user input continuously
- Uses pattern matching to handle commands
- Supports: "add [description]", "list", "complete [id]", "quit"
- Includes proper error handling for invalid input
```

**Prompt 2: Vector Search & Modify**
```
In Rust, how do I search through a Vec<Task> to find a task by ID 
and modify its completed field? Show me the idiomatic way using 
iterators and explain the borrowing involved.
```

**Prompt 3: File I/O**
```
Show me how to implement file persistence in Rust:
1. Save a Vec of structs to a text file (simple format)
2. Load it back on program start
3. Handle errors properly with Result<T, E>
4. Use a format that's easy to parse (like CSV or pipe-delimited)
```

### Expected Completion Time ⏱️
- Morning session: 2-3 hours
- Afternoon testing: 1 hour
- Documentation: 30 minutes
- **Total:** ~3.5-4 hours

### Success Criteria ✓
- [ ] User can interactively add tasks
- [ ] User can list tasks on command
- [ ] User can mark tasks complete by ID
- [ ] Tasks persist between program runs
- [ ] All features tested and working
- [ ] Code is well-commented
- [ ] AI prompts documented

---

## Questions to Explore

### Answered 
- ~~Why does Rust need Visual Studio tools on Windows?~~
  - Answer: Needs MSVC linker and Windows SDK for native compilation

- ~~What's the difference between &self and &mut self?~~
  - Answer: Immutable vs mutable borrowing

### Still Exploring 
- How does Rust's lifetime system work in detail?
- When should I use Box<T> vs regular values?
- What are the performance implications of String vs &str?
- How do I handle more complex error cases?
- What's the difference between panic! and Result?
- When should I use Option<T> vs Result<T, E>?

---

## Code Snippets for Reference

### Current main.rs Structure
```rust
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
}

fn main() {
    println!("🦀 Welcome to Rust Task Manager!");
    
    let mut manager = TaskManager::new();
    
    manager.add_task("Learn Rust basics".to_string());
    manager.add_task("Build task manager".to_string());
    manager.list_tasks();
    
    println!("Goodbye! 👋");
}
```

---

## Personal Notes & Observations

### Development Environment
- **OS:** Windows 11
- **Editor:** Currently using PowerShell + Notepad, VS Code planned for Day 2
- **Terminal:** Visual Studio Developer PowerShell
- **Project Location:** `C:\Users\alooh\Documents\rust-capstone\`

### Learning Style
- Hands-on coding works best for me
- Error-driven learning is effective with Rust
- AI prompts help bridge knowledge gaps quickly
- Documentation is clearer after writing actual code

### Motivation Level
High! The feeling of overcoming compilation errors and seeing "Hello, world!" was incredibly rewarding. Looking forward to building more features tomorrow.

---

## Summary

**Status:**  COMPLETE AND SUCCESSFUL

**Major Achievements:**
1. Full Rust toolchain installed and working
2. Debugged and fixed 2 significant installation issues
3. Created working task manager with add/list features
4. Understood core Rust concepts (ownership, borrowing, structs)
5. Comprehensive documentation of the learning process

**Lines of Code Written:** ~80 lines
**Compilation Errors Fixed:** 2 major, 1 warning
**Concepts Learned:** 6+ (structs, ownership, borrowing, Vec, String, pattern matching)

**Confidence Level:** 7/10
- Strong on basic syntax and structure
- Need more practice with ownership rules
- Ready to tackle interactive features tomorrow



---



---

## Appendix: Command Reference

### Rust & Cargo Commands Used
```powershell
# Check Rust version
rustc --version

# Check Cargo version
cargo --version

# Create new Rust project
cargo new project_name

# Build project (compile without running)
cargo build

# Build and run project
cargo run

# Check code without building
cargo check

# Build optimized release version
cargo build --release
```

### Project Navigation
```powershell
# Navigate to Documents
cd $HOME\Documents

# Create directory
mkdir folder_name

# Change directory
cd folder_name

# List contents
dir

# Go up one level
cd ..
```

---

## Appendix: Helpful Error Messages

### Example 1: Helpful Compiler Error
```
error[E0502]: cannot borrow `manager` as mutable because it is also borrowed as immutable
  --> src\main.rs:45:5
   |
44 |     let task = &manager.tasks[0];
   |                 ------- immutable borrow occurs here
45 |     manager.add_task("new".to_string());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

**What I Learned:**
Can't modify data while holding a reference to it. Rust prevents data races at compile time!

---

*This journal will be updated daily throughout the capstone project.*



