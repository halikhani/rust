# Week 2 — Ownership & types

Exercises from the Rust book chapters on ownership, borrowing, lifetimes, structs, and enums.

| Crate | Focus |
| ----- | ----- |
| `ownership` | Moves, references, mutable borrows |
| `lifetimes` | `'a` annotations, `longest` helper |
| `first_word` | String slices, `first_word(&str) -> &str` |
| `structs` | Defining and using custom structs |
| `enums` | `Option`, pattern matching, `Coin` variants |

Each crate is standalone. Run from its directory:

```bash
cd ownership && cargo run
cd first_word && cargo test
```
