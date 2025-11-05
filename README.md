# Mini KV Store 🦀

A beginner learning project implementing a very basic key-value store in Rust.

## What is this?

I'm learning Rust and wanted to understand the fundamentals of how databases work under the hood. This is my attempt at building the simplest possible key-value store from scratch.

**⚠️ This is NOT production-ready!** It's purely educational.

## Current Status

Started: November 5, 2025

Right now, this is just a HashMap wrapper with basic operations. Very much a work in progress as I learn Rust!

## What I've Implemented

- [x] Basic in-memory storage using HashMap
- [x] `set(key, value)` - Store a key-value pair
- [x] `get(key)` - Retrieve a value by key
- [x] `delete(key)` - Remove a key-value pair
- [x] Simple CLI interface

## What I Want to Learn Next

- [ ] Persisting data to disk (how do databases actually save data?)
- [ ] Handling concurrent access (multiple users at once)
- [ ] Log-structured merge trees (LSM) - sounds complicated but fascinating!
- [ ] Implementing a Write-Ahead Log (WAL)
- [ ] Compaction and garbage collection
- [ ] Maybe networking? Turning this into a server?

## Why Rust?

I've been reading that Rust is great for systems programming - memory safety without garbage collection. Perfect for learning database internals!

## Resources I'm Learning From

- Various blog posts on database architecture
- Rust documentation
- Articles about distributed systems
- Books on building data systems (looking for good ones!)

## Running This

```bash
cargo run
```

Then you can use commands like:
```
> set name Alice
> get name
Alice
> delete name
> get name
Key not found
```

## Notes

This is literally my first attempt at anything like this. The code is probably not idiomatic Rust yet - I'm still learning! Any feedback welcome 🙏

Built while learning Rust in November 2025 🦀
