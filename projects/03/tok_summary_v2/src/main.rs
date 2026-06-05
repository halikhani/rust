use clap::Parser;
use std::path::PathBuf;
use tok_summary_v2::{
    load_tokenizer, longest_tokens, merge_count, special_tokens, validate_tokenizer, vocab_size,
    TokSummaryError,
};

#[derive(Parser)]
#[command(name = "tok_summary_v2")]
struct Args {
    path: PathBuf,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), TokSummaryError> {
    let args = Args::parse();
    let json = load_tokenizer(&args.path)?;
    validate_tokenizer(&json)?;

    println!("Vocab size: {}", vocab_size(&json));
    println!("Merge count: {}", merge_count(&json));
    println!("10 longest tokens: {:?}", longest_tokens(&json, 10));

    for t in longest_tokens(&json, 10) {
        println!(" {} ({})", t, t.as_bytes().len());
    }

    println!("5 special tokens: {:?}", special_tokens(&json, 5));
    for t in special_tokens(&json, 5) {
        println!("  {t}");
    }

    Ok(())
}
