#[non_exhaustive] // this tells the compiler that the enum is not exhaustive, so we can add new variants to it without breaking the code
pub enum Device {
    Cpu,
    CUDA(usize),
    Metal(usize),
}


pub fn matmul(a: &[f32], b: &[f32], device: Device) -> Vec<f32> {
    match device {
        Device::Cpu => matmul_cpu(a, b),
        Device::CUDA(id) => {
            let _ = id;
            unimplemented!("CUDA matmul is not yet implemented");
        }
        Device::Metal(id) => {
            let _ = id;
            unimplemented!("Metal matmul is not yet implemented");
        }
    }
}

fn matmul_cpu(a: &[f32], b: &[f32]) -> Vec<f32> {
    assert_eq!(a.len(), b.len(), "Matrices must have the same dimensions");
    let n = (a.len() as f32).sqrt() as usize; // n x n matrix, 'as' is used to convert the result to an integer

    assert_eq!(n * n, a.len(), "slice length must be a perfect square");

    let mut c = vec![0.0; n * n];

    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += a[i * n + k] * b[k * n + j]; // row-major order, s
            }
            c[i * n + j] = sum;
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_matmul_2x2() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![5.0, 6.0, 7.0, 8.0];
        let result = matmul(&a, &b, Device::Cpu);
        assert_eq!(result, vec![19.0, 22.0, 43.0, 50.0]);
    }
}