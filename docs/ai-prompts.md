# AI Prompts Documentation

**Project:** Rust Task Manager CLI  
**Date:** November 30, 2025  
**Tool Used:** githubCopilot Chat

---

## Prompt 1: Rust Overview

**Timestamp:** Day 1, Morning

**Full Prompt:**
```
I'm learning Rust for a capstone project. Explain what makes Rust unique as 
a programming language. Cover:
1. What is Rust and what problems does it solve?
2. The ownership system in simple terms
3. Why companies like Discord and Microsoft use Rust
4. One specific real-world example

Keep it beginner-friendly but comprehensive.
```

**Key Points from AI Response:**
- Rust prevents memory bugs without garbage collection
- Ownership means each value has exactly one owner
- Borrowing allows temporary access without taking ownership
- Companies choose Rust for performance + safety
- Discord example: reduced latency from seconds to sub-millisecond

**My Evaluation:**
This gave me the foundational understanding I needed. The ownership explanation with the "lending a book" analogy made it click.



---

## Prompt 2: Interactive Command Loop

**Timestamp:** Day 1, Afternoon

**Full Prompt:**
```
Show me how to create an interactive command-line loop in Rust that:
- Reads user input continuously
- Uses pattern matching to handle commands
- Supports: "add [description]", "list", "complete [id]", "quit"
- Includes proper error handling for invalid input
```

**Key Points from AI Response:**
- Use `loop {}` for infinite loop
- Read with `io::stdin().read_line()`
- Pattern match with guards: `input if input.starts_with("add ")`
- Flush stdout to show prompt immediately
- Parse numbers with `.parse::<usize>()` and handle Result

**My Evaluation:**
Extremely practical. I used this structure directly in my application with minimal modifications.



---

## Prompt 3: Vector Search and Modification

**Timestamp:** Day 1, Afternoon

**Full Prompt:**
```
In Rust, how do I search through a Vec<Task> to find a task by ID and 
modify its completed field? Show me the idiomatic way using iterators 
and explain the borrowing involved.
```

**Key Points from AI Response:**
```rust
if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
    task.completed = true;
}
```
- `iter_mut()` creates mutable iterator
- `find()` returns `Option<&mut Task>`
- `if let` pattern matches the Some case
- Inside block, we have mutable reference to modify

**My Evaluation:**
This is the idiomatic Rust way. Understanding Option<T> and pattern matching together was a breakthrough.

---

## Prompt 4: File Persistence

**Timestamp:** Day 1, Late Afternoon

**Full Prompt:**
```
Show me how to implement file persistence in Rust:
1. Save a Vec of structs to a text file (simple format)
2. Load it back on program start
3. Handle errors properly with Result<T, E>
4. Use a format that's easy to parse (like pipe-delimited)
```

**Key Points from AI Response:**
- Use `fs::write()` and `fs::read_to_string()`
- Format: `id|description|completed`
- Return `io::Result<()>` for error handling
- Use `?` operator to propagate errors
- Parse with `.split('|')` and handle each field

**My Evaluation:**
Perfect practical example. I learned about Result<T, E> and the ? operator, which are fundamental to Rust.

---

## Prompt 5: Ownership in Methods

**Timestamp:** Day 1, Evening

**Full Prompt:**
```
Explain the difference between &self and &mut self in Rust method 
signatures. When should I use each? Give examples from a task manager 
application context.
```

**Key Points from AI Response:**
- `&self` = immutable borrow (read-only)
- `&mut self` = mutable borrow (can modify)
- Use `&self` for methods that only read data
- Use `&mut self` for methods that modify state
- Examples:
  - `list_tasks(&self)` - only reads
  - `add_task(&mut self)` - modifies vector

**My Evaluation:**
This resolved my biggest confusion. Now I understand why the compiler requires specific borrow types.

---

## Summary

**Total Prompts Used:** 5  


**Impact on Learning:**
AI prompts provided targeted, contextual explanations that accelerated understanding of complex concepts. The combination of conceptual explanations and practical code examples was highly effective.

**Best Practices Learned:**
1. Be specific in prompts - include context and constraints
2. Ask for examples in the domain you're working in
3. Request explanations of the "why" not just the "how"
4. Use AI for concepts, then verify with official docs
5. Document the prompts and responses for later reference
```

---

## **FINAL CHECKLIST - WHAT TO SUBMIT**

### Files You Need:
```
rust-capstone/
├── TOOLKIT.md ✅ (Main deliverable - copy from above)
├── docs/
│   └── ai-prompts.md ✅ (AI documentation - copy from above)
└── task_manager/
    ├── README.md ✅ (Project overview - copy from above)
    ├── Cargo.toml ✅ (Already exists)
    ├── src/
    │   └── main.rs ✅ (Your working code - already have it)
    └── tasks.txt (Generated automatically when you run)