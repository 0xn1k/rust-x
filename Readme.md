# 🦀 15-Day Rust Mastery Plan with Progress Tracker

> **Goal:** Become deeply proficient in Rust — from ownership and lifetimes to systems-level programming and optimization.

**Duration:** 15 Days  
**Time Commitment:** 4 hours/day  
**Objective:** Learn every essential concept, data structure, memory concept, and OS-level aspect of Rust.

---

## 🗓️ Phase 1 — Core Rust Foundations (Day 1–5)
Focus: Ownership, lifetimes, error handling, and pattern matching.

### Day 1 — Setup + Basics
- [ ] **Learn:** Toolchain, Cargo, rustc, project structure  
- [ ] **Code:** “Hello Rust”, variables, constants, types  
- [ ] **Reflect:** Why Rust is memory-safe without GC  
📘 *Read:* [The Rust Book Ch 1–3](https://doc.rust-lang.org/book/)
#### Notes / Challenges


### Day 2 — Ownership, Borrowing, Lifetimes
- [ ] **Learn:** Ownership rules, references, lifetimes  
- [ ] **Code:** Reverse a string without cloning  
- [ ] **Reflect:** Why double-free is impossible in Rust  
📘 *Read:* Ch 4
#### Notes / Challenges


### Day 3 — Structs, Enums, Pattern Matching
- [ ] **Learn:** Structs, enums, pattern matching  
- [ ] **Code:** Implement a simple calculator using enums and pattern matching  
- [ ] **Reflect:** Difference between `match`, `if let`, `while let`  
📘 *Read:* Ch 5–6
#### Notes / Challenges


### Day 4 — Collections + Error Handling
- [ ] **Learn:** Vec, HashMap, Option, Result  
- [ ] **Code:** Word frequency counter from a text file  
- [ ] **Reflect:** Difference between panic vs Result  
📘 *Read:* Ch 8–9
#### Notes / Challenges


### Day 5 — Modules + Packages + Traits
- [ ] **Learn:** Modules, crates, traits, generics  
- [ ] **Code:** Write a crate with 2 modules  
- [ ] **Reflect:** Traits are “interfaces with power”  
📘 *Read:* Ch 10–11
#### Notes / Challenges


---

## 🗓️ Phase 2 — Advanced Rust + Data Structures (Day 6–10)
Focus: Memory layout, generics, lifetimes, building data structures.

### Day 6 — Deep Dive into Generics + Traits
- [ ] **Learn:** Trait bounds, generic functions, blanket impls  
- [ ] **Code:** Implement `Summable` trait for Vec<i32>  
- [ ] **Reflect:** Trait objects vs generics  
📘 *Read:* Ch 10 + Rust by Example (Traits)
#### Notes / Challenges


### Day 7 — Smart Pointers & Memory Management
- [ ] **Learn:** Box, Rc, Arc, RefCell, interior mutability  
- [ ] **Code:** Build a linked list using Box and Option  
- [ ] **Reflect:** Compare `Rc<RefCell<T>>` to C++ shared_ptr  
📘 *Read:* Ch 15
#### Notes / Challenges


### Day 8 — Custom Data Structures
- [ ] **Learn:** Implement LinkedList, Stack, Queue manually  
- [ ] **Code:** Custom Stack with push/pop using generics  
- [ ] **Reflect:** Ownership in self-referential structs  
📘 *Read:* Ch 17
#### Notes / Challenges


### Day 9 — Lifetimes in Practice
- [ ] **Learn:** Lifetime annotations, static lifetime, struct lifetimes  
- [ ] **Code:** Struct holding a string reference  
- [ ] **Reflect:** Why lifetimes exist and how Rust infers them  
📘 *Read:* Ch 10.3 + Nomicon intro
#### Notes / Challenges


### Day 10 — File I/O, CLI Tools, Error Propagation
- [ ] **Learn:** std::fs, std::env, anyhow crate  
- [ ] **Code:** Simple todo CLI storing tasks in a file  
- [ ] **Reflect:** Error propagation and graceful handling  
📘 *Read:* Ch 9 + `anyhow` docs
#### Notes / Challenges


---

## 🗓️ Phase 3 — Systems + Performance (Day 11–15)
Focus: Concurrency, async, unsafe, OS-level concepts, optimization.

### Day 11 — Concurrency in Rust
- [ ] **Learn:** Threads, Channels, Arc<Mutex>, Send + Sync  
- [ ] **Code:** Parallel word counter using threads  
- [ ] **Reflect:** Ownership and thread safety  
📘 *Read:* Ch 16
#### Notes / Challenges


### Day 12 — Async Programming
- [ ] **Learn:** async/await, tokio basics, tasks  
- [ ] **Code:** Simple async HTTP fetcher  
- [ ] **Reflect:** Threads vs async tasks  
📘 *Read:* tokio.rs guides
#### Notes / Challenges


### Day 13 — Unsafe Rust + Memory Layout
- [ ] **Learn:** Raw pointers, unsafe blocks, FFI basics  
- [ ] **Code:** Function using raw pointer deref  
- [ ] **Reflect:** When to use unsafe and why  
📘 *Read:* Rustonomicon intro
#### Notes / Challenges


### Day 14 — Performance & Benchmarking
- [ ] **Learn:** Profiling, cargo bench, optimization flags  
- [ ] **Code:** Benchmark Vec vs LinkedList for insertion  
- [ ] **Reflect:** Zero-cost abstractions  
📘 *Read:* Rust Performance Book
#### Notes / Challenges


### Day 15 — Mini Project + Review
- [ ] **Project:** Choose one  
  - [ ] CLI File Compressor  
  - [ ] Multithreaded Web Scraper  
  - [ ] Tiny Key-Value Store  
- [ ] **Reflect:** Document your journey — what was hardest, what surprised you most.
📘 *Read:* Final review of The Rust Book  
#### Notes / Challenges


---

## 🧠 Tools to Use Daily
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Nomicon](https://doc.rust-lang.org/nomicon/)
- [Crates.io](https://crates.io/)

---

## 🏁 Beyond 15 Days
- [ ] Implement a memory allocator
- [ ] Build a tiny web server
- [ ] Learn WebAssembly (WASM) with Rust
- [ ] Publish your first crate on crates.io

---

> “Fearless concurrency and memory safety — mastered.” 🦀
