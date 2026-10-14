# mini-kvstore

An in-memory key-value store in Rust, with an interactive prompt. It was my first key-value store, started on November 5, 2025, and it is deliberately as small as it can be: a `HashMap<String, String>` behind four commands.

It has no dependencies, keeps nothing on disk, and serves a single user in a terminal. Everything is lost when it stops.

## Run it

Requires Rust 1.75 or later.

```bash
git clone https://github.com/whispem/mini-kvstore
cd mini-kvstore
cargo run
```

## Commands

| Command | Effect |
| --- | --- |
| `set <key> <value>` | Stores the value under the key, replacing any previous one. The value may contain spaces. |
| `get <key>` | Prints the value, or says the key is not found. |
| `delete <key>` | Removes the key and prints the value it held. |
| `count` | Prints the number of keys stored. |
| `quit` or `exit` | Leaves the prompt. |

## A session

```text
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

## How it works

The whole store is one struct:

```rust
struct MiniKVStore {
    data: HashMap<String, String>,
}
```

`set`, `get`, `delete` and `count` call `HashMap::insert`, `get`, `remove` and `len`. The prompt reads a line, splits it into words, and matches on the first one.

## Limits

- No persistence: the data lives only as long as the process.
- One user at a time, in the terminal; no network access.
- Keys and values are strings.
- No tests.

## What came next

Each of these limits became the starting point of a later project:

- [CSV-Key-Value-Store](https://github.com/whispem/CSV-Key-Value-Store): the data saved to a file.
- [Tiny-Log-Structured-KV-Store](https://github.com/whispem/Tiny-Log-Structured-KV-Store): an append-only log.
- [mini-kvstore-v2](https://github.com/whispem/mini-kvstore-v2): segments, checksums, compaction and an HTTP API.

## License

MIT. See [LICENSE](LICENSE).
