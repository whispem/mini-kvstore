# Mini KVStore 🦀

**A foundational in-memory key-value store built in Rust for learning system fundamentals**

[![Rust Version](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[About](#-about) •
[Quick Start](#-quick-start) •
[Usage](#-usage) •
[Learning Goals](#-learning-goals) •
[Roadmap](#-roadmap)

---

## 📚 About

Mini KV Store is a minimal, in-memory key-value database built from scratch in Rust. 
This project is part of my journey into understanding database internals and low-level system design—building the simplest possible storage engine to grasp core concepts before adding complexity.

**Started:** November 5, 2025  
**Philosophy:** Build clear, minimal systems. Understand every piece.

### Why this project?

This is intentionally the simplest possible key-value store:
- Pure in-memory storage (no persistence... yet!)
- Clean HashMap wrapper
- Interactive CLI for exploration
- Zero external dependencies
- Focus on clarity over features

Perfect for understanding fundamentals before diving into:
- Persistence layers
- Concurrency
- Write-ahead logs
- LSM trees
- All the "real database stuff"

---

## ✨ Features

### Current Implementation
- 🗄️ **In-memory storage** - HashMap-based key-value pairs
- ⚡ **O(1) operations** - Fast set, get, and delete
- 🎯 **String types** - Simple String keys and values
- 💬 **Interactive CLI** - REPL for experimentation
- 📊 **Count command** - See how many items stored
- 🦀 **Pure Rust** - No external dependencies

### Design Philosophy
- **Clarity first** - Every line is understandable
- **Minimal abstractions** - See exactly what's happening
- **Educational** - Built for learning, not production
- **Foundation** - Base for more complex implementations

---

## 🚀 Quick Start

### Prerequisites

- **Rust 1.75+** - [Install Rust](https://rustup.rs/)

### Installation

```bash
# Clone the repository
git clone https://github.com/whispem/mini-kvstore
cd mini-kvstore

# Run the project
cargo run
```

### Example Session

```bash
🦀 Mini KV Store - Learning Project
Commands: set <key> <value> | get <key> | delete <key> | count | quit

> set name Alice
✓ Set 'name' = 'Alice'

> set language Rust
✓ Set 'language' = 'Rust'

> get name
Alice

> count
Store contains 2 items

> delete name
✓ Deleted 'name' (was: 'Alice')

> get name
Key 'name' not found

> quit
Goodbye! 🦀
```

---

## 💻 Usage

### Available Commands

#### Set a Key-Value Pair
```bash
> set <key> <value>
```
Stores a new key-value pair or updates an existing key.

**Example:**
```bash
> set user Alice
✓ Set 'user' = 'Alice'

> set user Bob
✓ Set 'user' = 'Bob'  # Updates existing key
```

#### Get a Value
```bash
> get <key>
```
Retrieves the value for a key.

**Example:**
```bash
> get user
Bob
```

#### Delete a Key
```bash
> delete <key>
```
Removes a key-value pair from the store.

**Example:**
```bash
> delete user
✓ Deleted 'user' (was: 'Bob')
```

#### Count Items
```bash
> count
```
Shows how many key-value pairs are currently stored.

#### Exit
```bash
> quit
```
or
```bash
> exit
```
Exits the application.

---

## 🏗️ Architecture

### Simple Design

```
┌─────────────────────────┐
│    Interactive CLI      │
│   (User Commands)       │
└───────────┬─────────────┘
            │
            ▼
┌─────────────────────────┐
│   MiniKVStore Struct    │
│                         │
│  ┌───────────────────┐ │
│  │ HashMap           │ │
│  │ <String, String>  │ │
│  └───────────────────┘ │
└─────────────────────────┘
```

### How It Works

The entire "database" is just a Rust struct wrapping a HashMap:

```rust
struct MiniKVStore {
    data: HashMap<String, String>,
}
```

**Operations:**
- `set()` - Calls `HashMap::insert()`
- `get()` - Calls `HashMap::get()`
- `delete()` - Calls `HashMap::remove()`
- `count()` - Returns `HashMap::len()`

That's it! Pure in-memory, no persistence, no complexity.

### What This Teaches

- **Core data structure** - HashMap is the heart of all KV stores
- **API design** - Simple CRUD interface
- **Rust basics** - Ownership, borrowing, Option types
- **REPL pattern** - Interactive command loop
- **Starting point** - Foundation for adding persistence, concurrency, etc.

---

## 📚 Learning Goals

### Fundamentals Covered

**Data Structures:**
- HashMap as the core storage mechanism
- String handling in Rust
- Option types for missing keys

**Rust Concepts:**
- Struct methods and `impl` blocks
- Ownership of HashMap entries
- Mutable vs immutable references
- Pattern matching with `match`
- `Option<T>` for nullable returns

**System Design:**
- Separation of storage and interface
- Command parsing and validation
- User interaction patterns
- Error handling basics

### What's Intentionally Missing

This project doesn't have:
- ❌ Persistence (data lost on exit)
- ❌ Concurrency (single-threaded only)
- ❌ Network access (local only)
- ❌ Type flexibility (String only)
- ❌ Tests (yet!)
- ❌ Error recovery

**Why?** These are the next learning steps! Start simple, add complexity incrementally.

---

## 🗺️ Roadmap

### Current Status ✅

**Completed (November 2025):**
- [x] In-memory HashMap storage
- [x] Basic CRUD operations (set, get, delete)
- [x] Interactive CLI/REPL
- [x] Count command
- [x] Clean code with comments

### Next Steps (Learning Plan) 📋

**Phase 1 - Fundamentals:**
- [ ] Add unit tests
- [ ] Better error messages
- [ ] Support for values with spaces
- [ ] Command history in REPL

**Phase 2 - Persistence:**
- [ ] Save to disk on exit
- [ ] Load from disk on startup
- [ ] Explore serialization formats

**Phase 3 - Advanced:**
- [ ] Write-ahead log (WAL)
- [ ] Basic compaction
- [ ] Concurrent access handling
- [ ] LSM-tree exploration

**Phase 4 - Production Features:**
- [ ] Network interface (TCP server)
- [ ] Binary protocol
- [ ] Benchmarking suite
- [ ] Proper documentation

> 💡 **Note:** Many of these features are already implemented in [mini-kvstore-v2](https://github.com/whispem/mini-kvstore-v2)—my production-ready evolution of this project!

---

## 🤔 Design Decisions

### Why In-Memory Only?

Starting with pure in-memory storage teaches:
- The core HashMap operations
- How state flows through a system
- What a minimal API looks like

Without the complexity of:
- File I/O
- Serialization
- Crash recovery
- Write amplification

Once these basics are solid, persistence becomes the natural next step.

### Why HashMap?

HashMap is the foundation of every key-value store:
- **O(1) average-case** for all operations
- **Built-in to Rust** - well-tested, optimized
- **Simple mental model** - just a lookup table

Understanding HashMap deeply is essential before exploring more complex structures like LSM trees or B-trees.

### Why So Simple?

This project prioritizes **learning** over **features**:
- Every line of code is understandable
- No "magic" or hidden complexity
- Easy to modify and experiment
- Clear progression to next features

Complexity will come naturally as needs arise.

---

## 🦀 Why Rust?

Rust is perfect for learning system fundamentals:

- **Memory safety** - Learn without segfaults
- **Ownership model** - Understand resource management
- **Explicit types** - See exactly what data you're handling
- **Great errors** - Compiler teaches best practices
- **Performance** - Fast even in learning projects

This project uses:
- `HashMap` from `std::collections`
- `io` module for CLI interaction
- Basic pattern matching
- Option types for nullable data

Pure, idiomatic Rust with no external dependencies.

---

## 📚 Learning Resources

### Key-Value Stores
- [Database Internals](https://www.databass.dev/) by Alex Petrov
- [Designing Data-Intensive Applications](https://dataintensive.net/) by Martin Kleppmann
- [Build Your Own Database](https://build-your-own.org/database/)

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/) - Start here
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Learning exercises

### My Learning Path
This project is part of a progression:
1. **Mini KVStore** ← You are here (Basics)
2. [Tiny Log-KV](https://github.com/whispem/tiny-log-kv) - Add persistence
3. [Mini KVStore v2](https://github.com/whispem/mini-kvstore-v2) - Production features

---

## 🤝 Contributing

This is a learning project, but feedback is always welcome!

### Ways to Help

- 🐛 **Spot issues** - See something wrong? Let me know
- 💡 **Suggest resources** - Know great learning materials?
- 📖 **Improve docs** - Make explanations clearer
- 🎓 **Share your version** - Fork and experiment!

### For Fellow Learners

Feel free to:
- Fork this repo and add features
- Implement items from the roadmap
- Share what you learned
- Ask questions via issues

---

## 📜 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

---

## 👤 Author

**Em' ([@whispem](https://github.com/whispem))**

From literature & languages to building storage engines. This project represents my first steps into database fundamentals—starting with the simplest possible implementation, understanding every piece, then growing from there.

> *"The best way to learn is to build."*

---

## 🌟 Part of My Learning Journey

This is project #3 in my storage systems learning path:

1. [CSV-KV Store](https://github.com/whispem/CSV-Key-Value-Store) - File persistence basics
2. [Tiny Log-KV](https://github.com/whispem/tiny-log-kv) - Append-only logs
3. **Mini KVStore** ← You are here (In-memory foundations)
4. [Mini KVStore v2](https://github.com/whispem/mini-kvstore-v2) - Production-ready implementation

Each project teaches new concepts while building on the previous ones! 🚀

---

## 📬 Contact

- 🐛 **Issues:** [GitHub Issues](https://github.com/whispem/mini-kvstore/issues)
- 💬 **Suggestions:** Open a discussion!
- 📧 **Email:** contact.whispem@gmail.com

---

## 💭 TODO Notes (From Code)

```rust
// TODO for learning:
// - Add persistence: save to a file, load on startup
// - Handle errors better (use Result types)
// - Add tests!
// - Learn about more advanced data structures (LSM trees, B-trees)
// - Figure out how to handle concurrent access safely
// - Maybe add a simple network interface?
//
// This is just the beginning! 🚀
```

These TODOs represent the natural evolution from this foundation to a real database system. Each one is a new learning opportunity!

---

**Built with ❤️ and curiosity in Rust**

[⬆ Back to Top](#mini-kv-store-)
