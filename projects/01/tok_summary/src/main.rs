use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)] // clap macro generates parsing logic for Args::parse() to read tok_summary path from command line
#[command(name="tok_summary")]
struct Args {
    // path to tokenizer.json
    path: PathBuf,
}

fn main() {
    let args = Args::parse(); // builds Args or exits with a usage message on bad input
    let text = fs::read_to_string(&args.path).expect("Failed to read file");
    let json: serde_json::Value =
        serde_json::from_str(&text).expect("Failed to parse JSON");

    println!("Vocab size: {}", vocab_size(&json));
    println!("Merge count: {}", merge_count(&json));
    println!("10 longest tokens: {:?}", longest_tokens(&json, 10));

    for t in longest_tokens(&json, 10) {
        println!(" {} ({})", t, t.as_bytes().len());
    }

    println!("5 special tokens: {:?}", special_tokens(&json, 10));
    for t in special_tokens(&json, 5) {
        println!("  {t}");
    }
}


fn vocab_size(json: &serde_json::Value) -> usize {
    json.get("model")
        .and_then(|m| m.get("vocab"))
        .and_then(|v| v.as_object())
        .map(|a| a.len())
        .unwrap_or(0) // if any steps fail, return 0
}

fn merge_count(json: &serde_json::Value) -> usize {
    json.get("model")
        .and_then(|m| m.get("merges"))
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0)
}

fn longest_tokens(json: &serde_json::Value, n: usize) -> Vec<String> {
    let mut tokens: Vec<String> = json.get("model")
        .and_then(|m| m.get("vocab"))
        .and_then(|v| v.as_object())
        .map(|obj| obj.keys().cloned().collect())
        .unwrap_or_default();
    
    tokens.sort_by_key(|t| std::cmp::Reverse(t.as_bytes().len())); // sort descending by length
    tokens.truncate(n); // truncate to n longest tokens
    tokens
}


fn special_tokens(json: &serde_json::Value, n: usize) -> Vec<String> {
    json.get("added_tokens")
        .and_then(|a| a.as_array())
        .map(|arr| {
            arr.iter()
                .take(n)
                .filter_map(|t| t.get("content").and_then(|c| c.as_str()))
                .map(str::to_string)
                .collect() // builds a Vector inside a map() function
        })
        .unwrap_or_default()
}
