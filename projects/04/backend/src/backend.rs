pub trait Backend {
    /// C = A @ B, row-major.
    /// A: m×k, B: k×n, returns C: m×n (length m*n).
    fn matmul_f32(&self, a: &[f32], b: &[f32], m: usize, k: usize, n: usize) -> Vec<f32>;

    /// Elementwise a + b (same length)
    fn add_f32(&self, a: &[f32], b: &[f32]) -> Vec<f32>;

    fn name(&self) -> &'static str;
}
