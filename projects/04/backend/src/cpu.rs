use crate::backend::Backend;

#[derive(Debug, Clone, Default, Copy)]
pub struct CpuBackend;

impl Backend for CpuBackend {
    fn matmul_f32(&self, a: &[f32], b: &[f32], m: usize, k: usize, n: usize) -> Vec<f32> {
        assert_eq!(a.len(), m * k, "a.len() != m * k");
        assert_eq!(b.len(), k * n, "b.len() != k * n");

        let mut c = vec![0.0; m * n];

        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for l in 0..k {
                    sum += a[i * k + l] * b[l * n + j];
                }
                c[i * n + j] = sum;
            }
        }
        c
    }

    fn add_f32(&self, a: &[f32], b: &[f32]) -> Vec<f32> {
        assert_eq!(a.len(), b.len(), "a.len() != b.len()");
        a.iter().zip(b.iter()).map(|(&x, &y)| x + y).collect()
    }

    fn name(&self) -> &'static str {
        "cpu"
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matmul_test() {
        let backend = CpuBackend;
        let a = vec![1.0, 2.0, 3.0, 4.0]; // 2×2
        let b = vec![5.0, 6.0, 7.0, 8.0]; // 2×2
        let c = backend.matmul_f32(&a, &b, 2, 2, 2);
        assert_eq!(c, vec![19.0, 22.0, 43.0, 50.0]);
    }

    #[test]
    fn matmul_non_square() {
        let backend = CpuBackend;
        // A: 2 x 3, B: 3 x 2, C: 2 x 2
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
        let c = backend.matmul_f32(&a, &b, 2, 3, 2);
        assert_eq!(c, vec![58.0, 64.0, 139.0, 154.0]);
    }

    #[test]
    fn add_f32_works() {
        let backend = CpuBackend;
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let c = backend.add_f32(&a, &b);
        assert_eq!(c, vec![5.0, 7.0, 9.0]);
    }
}
