# Rust Learning Resources

---

## Official documentation

| Resource | URL | When to use |
| -------- | --- | ----------- |
| **The Rust Book** | [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) | Start here; read alongside week 1–4 |
| **Rust by Example** | [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/) | Quick snippets when the book feels heavy |
| **Standard library docs** | [doc.rust-lang.org/std](https://doc.rust-lang.org/std/) | Daily reference |
| **Rust Reference** | [doc.rust-lang.org/reference](https://doc.rust-lang.org/reference/) | Precise language rules |
| **Cargo Book** | [doc.rust-lang.org/cargo](https://doc.rust-lang.org/cargo/) | Workspaces, dependencies, publishing |
| **Rustonomicon** | [doc.rust-lang.org/nomicon](https://doc.rust-lang.org/nomicon/) | `unsafe`, FFI, layout (weeks 8+) |
| **Async Book** | [rust-lang.github.io/async-book](https://rust-lang.github.io/async-book/) | `async`/`await`, futures (week 7) |

---

## Books (recommended order)

1. **The Rust Programming Language** — Klabnik & Nichols. Free online; the
   default starting point.
2. **Rustlings** + small projects — see [Practice](#practice) below.
3. **Rust for Rustaceans** — Jon Gjengset. Lifetimes, traits, async, `unsafe`;
   best after you can write basic programs.
4. **Programming Rust, 2nd ed.** — Blandy / Orendorff / Tindall. Strong on
   concurrency and the standard library.
5. **Asynchronous Programming in Rust** — official async book (see table above).
6. **Rust Atomics and Locks** — Mara Bos. Free at
   [marabos.nl/atomics](https://marabos.nl/atomics/). Memory ordering when you
   hit threads/`Arc`/lock-free code.


Workflow while learning:

```bash
cargo new my_project
# edit src/main.rs or src/lib.rs
cargo build
cargo run
cargo test
cargo clippy -- -D warnings
cargo fmt
```

---

## Tooling to install

```bash
# Toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy rust-analyzer rust-src

# Optional but useful
rustup toolchain install nightly   # for experiments / some crates
cargo install cargo-watch cargo-expand cargo-nextest
cargo install hyperfine            # CLI benchmarking
```

**`cargo` commands you will use constantly**

| Command | Purpose |
| ------- | ------- |
| `cargo new foo` / `cargo new --lib foo` | New binary / library crate |
| `cargo build [--release]` | Compile (debug or optimized) |
| `cargo run -- arg1 arg2` | Build and run; args after `--` go to your program |
| `cargo test` | Run `#[test]` and doctests |
| `cargo clippy -- -D warnings` | Lint; treat warnings as errors |
| `cargo fmt` | Format code |
| `cargo doc --open` | Build and open API docs for your crate |
| `cargo expand` | See macro-expanded code (after installing `cargo-expand`) |

---
