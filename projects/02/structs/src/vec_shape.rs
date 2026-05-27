pub struct Shape(Vec<usize>);
impl Shape {
    pub fn new<S: Into<Vec<usize>>>(s: S) -> Self {
        Shape(s.into())
    }
    pub fn rank(&self) -> usize {
        self.0.len()
    }
    pub fn numel(&self) -> usize {
        self.0.iter().product()
    }
    pub fn as_slice(&self) -> &[usize] {
        &self.0
    }
}

impl From<Vec<usize>> for Shape {
    fn from(v: Vec<usize>) -> Self {
        Shape(v)
    }
}

impl From<&[usize]> for Shape {
    fn from(v: &[usize]) -> Self {
        Shape(v.to_vec())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_vec_preserves_dims() {
        let s = Shape::from(vec![1, 2, 3]);
        assert_eq!(s.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn from_slice_preserves_dims() {
        let s = Shape::from(&[1, 2, 3][..]);
        assert_eq!(s.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn rank_counts_dimensions() {
        let s = Shape::from(vec![1, 2, 3]);
        assert_eq!(s.rank(), 3);
    }

    #[test]
    fn numel_counts_elements() {
        let s = Shape::from(vec![1, 2, 3]);
        assert_eq!(s.numel(), 6);
    }

    #[test]
    fn empty_shape_has_rank_0() {
        let s = Shape::new(Vec::<usize>::new());
        assert_eq!(s.rank(), 0);
    }
}
