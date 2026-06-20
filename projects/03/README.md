# Week 3 — Collections & ML foundations

Rust patterns applied to tokenizer tooling and inference-adjacent types.

| Crate | What it does |
| ----- | ------------ |
| `collections` | Vectors, HashMap, enums in practice |
| `dtype` | `DType` enum (F32, F16, BF16, …), byte-size helpers |
| `safetensors_header` | Parse safetensors file headers (length prefix + JSON) |
| `device_dispatch` | `matmul` routed by `Device` (CPU stub) |
| `sampling` | `SamplingParams` / `SamplingMode` with validation |
| `tok_summary_v2` | Refactored tokenizer CLI with proper error types |

## Run

```bash
cd tok_summary_v2 && cargo run -- path/to/tokenizer.json
cd safetensors_header && cargo test
cd sampling && cargo test
```
