use std::{fs, path::Path, path::PathBuf};



// Debug for {:?} style printing
// thiserror::Error implements Display and std::error::Error from the #[error] attribute
#[derive(Debug, thiserror::Error)]
pub enum TokSummaryError {
    #[error("Failed to read {path}: {source}")]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Invalid JSON in {path}: {source}")]
    BadJson {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[error("malformed tokenizer.json: missing or invalid `{field}`")]
    Malformed { field: &'static str },
}

pub fn load_tokenizer(path: &Path) -> Result<serde_json::Value, TokSummaryError> {
    let text = fs::read_to_string(path).map_err(|e| TokSummaryError::ReadFile{
        path: path.to_path_buf(),
        source: e,
    })?;

    let json = serde_json::from_str(&text).map_err(|source| TokSummaryError::BadJson {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(json)

}


pub fn validate_tokenizer(json: &serde_json::Value) -> Result<(), TokSummaryError> {
    let model = json.get("model").ok_or(TokSummaryError::Malformed{ field: "model" })?;
    model.get("vocab")
        .and_then(|v| v.as_object())
        .ok_or(TokSummaryError::Malformed{ field: "model.vocab" })?;

    Ok(())
}

pub fn vocab_size(json: &serde_json::Value) -> usize {
    json.get("model")
        .and_then(|m| m.get("vocab"))
        .and_then(|v| v.as_object())
        .map(|a| a.len())
        .unwrap_or(0)
}

pub fn merge_count(json: &serde_json::Value) -> usize {
    json.get("model")
        .and_then(|m| m.get("merges"))
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0)
}

pub fn longest_tokens(json: &serde_json::Value, n: usize) -> Vec<String> {
    let mut tokens: Vec<String> = json
        .get("model")
        .and_then(|m| m.get("vocab"))
        .and_then(|v| v.as_object())
        .map(|obj| obj.keys().cloned().collect())
        .unwrap_or_default();

    tokens.sort_by_key(|t| std::cmp::Reverse(t.as_bytes().len()));
    tokens.truncate(n);
    tokens
}

pub fn special_tokens(json: &serde_json::Value, n: usize) -> Vec<String> {
    json.get("added_tokens")
        .and_then(|a| a.as_array())
        .map(|arr| {
            arr.iter()
                .take(n)
                .filter_map(|t| t.get("content").and_then(|c| c.as_str()))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}