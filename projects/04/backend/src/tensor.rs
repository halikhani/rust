pub struct Tensor<T> { data: Vec<T>, shape: Vec<usize> }
impl<T: Copy + Default> Tensor<T> {
    pub fn zeros(shape: &[usize]) -> Self {
        let n = shape.iter().product();
        Self {data: vec![T::default(); n], shape: shape.to_vec(),}
    }
    pub fn fill(shape: &[usize], v: T) -> Self {
        let n = shape.iter().product();
        Self {data: vec![v; n], shape: shape.to_vec()} // vec![v; n] needs Copy, rust duplicates v for every slot so it needs Copy trait
    }
    pub fn shape(&self) -> &[usize] {
        &self.shape // why not self.shape? because self.shape is a Vec<usize> and we want to return a slice of usize
    }
    pub fn numel(&self) -> usize {
        self.shape.iter().product() // or self.data.len()
    }
}
impl<T: Copy + std::ops::Add<Output = T>> Tensor<T> {
    // why Add<Output = T> trait? because we want to add two tensors of the same type and shape and return a tensor of the same type and shape
    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(self.shape, other.shape, "Shape mismatch");
        let data = self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b) // why &? because we want to borrow the data from the tensors
            .collect();

        Self {
            data,
            shape: self.shape.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use half::f16;


    #[test]
    fn zeros_and_fill_f32() {
        let z = Tensor::<f32>::zeros(&[2, 3]);
        assert_eq!(z.shape(), &[2, 3]);
        assert_eq!(z.numel(), 6);
        assert!(z.data.iter().all(|&x| x == 0.0));

        let f = Tensor::<f32>::fill(&[2, 3], 1.0);
        assert_eq!(f.shape(), &[2, 3]);
        assert_eq!(f.numel(), 6);
        assert!(f.data.iter().all(|&x| x == 1.0));
    }

    #[test]
    fn zeros_and_fill_i8() {
        let z = Tensor::<i8>::zeros(&[4]);
        assert_eq!(z.data, vec![0, 0, 0, 0]);

        let f = Tensor::fill(&[2, 2], 6);
        assert_eq!(f.data, vec![6, 6, 6, 6]);
    }


    // impl block 2 copy and add
    #[test]
    fn add_f32() {
        let a = Tensor::<f32>::fill(&[2, 3], 1.0);
        let b = Tensor::<f32>::fill(&[2, 3], 2.0);
        let c = a.add(&b);
        assert_eq!(c.data, vec![3.0, 3.0, 3.0, 3.0, 3.0, 3.0]);
    }

    #[test]
    fn add_f16() {
        let a = Tensor::<f16>::fill(&[2, 3], f16::from_f32(1.0));
        let b = Tensor::<f16>::fill(&[2, 3], f16::from_f32(2.0));
        let c = a.add(&b);
        assert_eq!(c.data, vec![f16::from_f32(3.0); 6]);
    }

    #[test]
    fn add_i8() {
        let a = Tensor::<i8>::fill(&[2, 3], 1_i8);
        let b = Tensor::<i8>::fill(&[2, 3], 2_i8);
        let c = a.add(&b);
        assert_eq!(c.data, vec![3, 3, 3, 3, 3, 3]);
    }
}