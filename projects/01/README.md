# Week 1 — Cargo & basics

First steps with workspaces, library vs binary crates, and a small CLI.

| Crate | Type | What it does |
| ----- | ---- | ------------ |
| `lib_hello` | library | Simple `greeting()` function |
| `bin_hello` | binary | Prints greeting; includes fizzbuzz + tests |
| `tok_summary` | binary + lib | Reads `tokenizer.json`, prints vocab size, merges, longest & special tokens |

## Run

```bash
cargo test                              # all workspace members
cargo run -p bin_hello
cargo run -p tok_summary -- path/to/tokenizer.json
```
