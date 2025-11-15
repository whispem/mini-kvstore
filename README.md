# Mini KV Store 🦀

A small, exploratory key–value store written in Rust.  
This project is part of my learning journey into system fundamentals, storage design, and the low-level concepts behind database internals.  
The goal: build a clear, minimal system while understanding each piece step by step.

---

## 🧩 What This Is

A tiny key–value store implemented from scratch, currently running as a simple CLI.  
It’s intentionally minimal and focused — built to learn:

- how in-memory stores work,
- how state flows through simple systems,
- how to evolve small tools toward more realistic storage behavior.

This project prioritizes clarity and readability over features.

---

## ⚙️ Installation & Running

Clone the repository:

```bash
git clone https://github.com/whispem/mini-kvstore
cd mini-kvstore
```

Run the CLI:

```bash
cargo run
```

Example session:

```
> set name Alice
> get name
Alice
> delete name
> get name
Key not found
```

---

## 📌 Current Status

**Started:** November 5, 2025  
**Current phase:** building and refining a minimal in-memory store

Implemented so far:

- In-memory storage using `HashMap`
- `set(key, value)` — store/update
- `get(key)` — retrieve
- `delete(key)` — remove
- Interactive CLI

The project is intentionally small, but each feature is an opportunity to understand how real-world systems behave.

---

## 📈 Learning Roadmap

Gradually growing this project by exploring real database fundamentals:

- Persistence: writing data to disk  
- Write-Ahead Log (WAL)  
- Basic compaction / cleanup  
- Concurrency handling  
- LSM-tree concepts  
- *(Maybe later)* turn this into a tiny networked KV server

Each item will be implemented in small, well-defined steps — to keep the code understandable and the learning intentional.

---

## 🦀 Why Rust?

Rust gives me:

- memory safety without GC  
- explicit control over state and ownership  
- a great mental model for system boundaries  
- a community that values clarity, correctness, and minimalism

Perfect for learning how storage systems work under the hood.

---

## 📚 Resources I’m Learning From

- Rust documentation  
- Blog posts on storage engines and database internals  
- Articles on WAL, LSM trees, durability, data layout  
- Notes on distributed systems fundamentals  
- Writing on “small, clear system design” principles  

I update this list as I learn.

---

## 🗒️ Notes

This is one of my first Rust learning projects — the code evolves as I understand the ecosystem better.  
Feedback, suggestions, or reading recommendations are always welcome. 🙏

---

Built while exploring Rust and system fundamentals — November 2025 🦀

---

If you spot anything that could be written in a more idiomatic or elegant Rust style, I’m always curious to understand why.