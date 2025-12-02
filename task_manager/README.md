# Rust Task Manager CLI

A command-line task management application built with Rust for the Moringa School GenAI Capstone Project.

## Features

-  Add tasks with descriptions
-  List all tasks with completion status
-  Mark tasks as complete by ID
-  Persistent storage (saves between sessions)
-  Interactive command-line interface

## Quick Start

### Prerequisites

- Rust 1.83+ (install from https://rustup.rs)
- Windows: Visual Studio Build Tools 2022 with MSVC compiler

### Run the Application
```bash
cargo run
```

### Commands
```
add <description>    # Add a new task
list                 # Show all tasks
complete <id>        # Mark task as done
quit                 # Save and exit
```

### Example Usage
```
 Welcome to Rust Task Manager!
Commands: add <description>, list, complete <id>, quit

> add Learn Rust basics
✓ Task added successfully!

> add Build CLI application
✓ Task added successfully!

> list

=== Your Tasks ===
○ [1] Learn Rust basics
○ [2] Build CLI application

> complete 1
✓ Task 1 marked as completed!

> quit
Tasks saved. Goodbye! 
```

## Project Structure
```
task_manager/
├── src/
│   └── main.rs       # Application code
├── Cargo.toml        # Project configuration
└── tasks.txt         # Data file (generated)
```

## Author

**[Harrison Onyango]**  
Moringa School - GenAI Capstone Project  
November 2025

## Documentation

For complete installation guide, troubleshooting, and learning resources, see [TOOLKIT.md](../TOOLKIT.md)