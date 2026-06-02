use dtype::{bytes_for, DType};

fn main() {
    for s in ["f32", "fp16", "bfloat16", "i4"] {
        if let Some(dt) = DType::parse_str(s) {
            println!("{s} -> {dt} ({} bits, {} bytes for 3 elements)", dt.size_in_bits(), bytes_for(3, dt));
        }
    }
}
