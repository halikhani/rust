# Week 4 — CLI & backend

Classic `minigrep` plus a small inference backend crate.

| Crate | What it does |
| ----- | ------------ |
| `minigrep` | File search CLI (case-sensitive / insensitive) |
| `backend` | `Tensor`, CPU matmul, `top_k_sort` vs `top_k_heap` |

## Run

```bash
# minigrep
cd minigrep
cargo run -- to poem.txt
cargo run -- to poem.txt -- -i

# backend
cd backend
cargo test
cargo bench --bench top_k
```

`top_k` benchmarks compare O(n log n) full sort vs O(n log k) min-heap selection.
