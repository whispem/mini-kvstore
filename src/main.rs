// My first attempt at building a key-value store!
// This is very basic - just a HashMap wrapper really.
// But it helps me understand the concepts.

use std::collections::HashMap;
use std::io::{self, Write};

// This is our "database" - just a struct wrapping a HashMap
// In a real database, this would persist to disk, handle concurrency, etc.
// But for now, it's just in memory!
struct MiniKVStore {
    // Using String for both keys and values to keep it simple
    // A real KV store would be more flexible with data types
    data: HashMap<String, String>,
}

impl MiniKVStore {
    // Create a new empty store
    fn new() -> Self {
        MiniKVStore {
            data: HashMap::new(),
        }
    }

    // Store a key-value pair
    // Returns the old value if the key existed, otherwise None
    fn set(&mut self, key: String, value: String) -> Option<String> {
        self.data.insert(key, value)
    }

    // Get a value by its key
    // Returns None if the key doesn't exist
    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    // Delete a key-value pair
    // Returns the value that was deleted, or None if key didn't exist
    fn delete(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    // Just for fun - show how many items we have
    fn count(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    println!("🦀 Mini KV Store - Learning Project");
    println!("Commands: set <key> <value> | get <key> | delete <key> | count | quit");
    println!();

    let mut store = MiniKVStore::new();

    loop {
        // Print prompt
        print!("> ");
        // Flush to make sure prompt appears before input
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Split input into words
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        // Match on the command (first word)
        match parts[0] {
            "set" => {
                if parts.len() < 3 {
                    println!("Usage: set <key> <value>");
                    continue;
                }
                let key = parts[1].to_string();
                // Join all remaining parts as the value (in case it has spaces)
                let value = parts[2..].join(" ");
                
                store.set(key.clone(), value.clone());
                println!("✓ Set '{}' = '{}'", key, value);
            }
            "get" => {
                if parts.len() < 2 {
                    println!("Usage: get <key>");
                    continue;
                }
                let key = parts[1];
                
                match store.get(key) {
                    Some(value) => println!("{}", value),
                    None => println!("Key '{}' not found", key),
                }
            }
            "delete" => {
                if parts.len() < 2 {
                    println!("Usage: delete <key>");
                    continue;
                }
                let key = parts[1];
                
                match store.delete(key) {
                    Some(value) => println!("✓ Deleted '{}' (was: '{}')", key, value),
                    None => println!("Key '{}' not found", key),
                }
            }
            "count" => {
                println!("Store contains {} items", store.count());
            }
            "quit" | "exit" => {
                println!("Goodbye! 🦀");
                break;
            }
            _ => {
                println!("Unknown command: {}", parts[0]);
                println!("Available: set, get, delete, count, quit");
            }
        }
    }
}

// TODO for learning:
// - Add persistence: save to a file, load on startup
// - Handle errors better (use Result types)
// - Add tests!
// - Learn about more advanced data structures (LSM trees, B-trees)
// - Figure out how to handle concurrent access safely
// - Maybe add a simple network interface?
//
// This is just the beginning! 🚀
