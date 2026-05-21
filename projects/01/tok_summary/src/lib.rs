/// Returns the number of entries in `model.vocab`.
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use tok_summary::vocab_size;
///
/// let j = json!({
///     "model": {
///         "vocab": { "a": 0, "b": 1, "c": 2 }
///     }
/// });
/// assert_eq!(vocab_size(&j), 3);
/// ```
pub fn vocab_size(json: &serde_json::Value) -> usize {
    json.get("model")
        .and_then(|m| m.get("vocab"))
        .and_then(|v| v.as_object())
        .map(|a| a.len())
        .unwrap_or(0)
}