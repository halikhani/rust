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


pub struct TensorView<'a> {
    data:    &'a [f32],
    shape:   &'a [usize],
    strides: &'a [usize],
}
impl<'a> TensorView<'a> {
    pub fn sum(&self) -> f32 {
        self.data.iter().sum()
    }

    pub fn numel(shape: &[usize]) -> usize {
        shape.iter().product()
    }
    pub fn get(&self, idx: &[usize]) -> Option<f32> {
        // check if dims of idx matches shape of tensor
        if idx.len() != self.shape.len() {
            return None;
        }
        // check if idx is within bounds
        for (i, dim) in idx.iter().zip(self.shape.iter()) {
            if i >= dim {
                return None;
            }
        }

        let offset: usize = idx
            .iter()
            .zip(self.strides.iter())
            .map(|(i, s)| i * s)
            .sum();

        self.data.get(offset).copied()
    }

    pub fn contiguous_strides(shape: &[usize]) -> Vec<usize> {
        let mut strides = vec![1; shape.len()];
        for i in (0..shape.len().saturating_sub(1)).rev() {
            strides[i] = strides[i + 1] * shape[i + 1];
        }
        strides
    } 

    pub fn reshape(&self, new_shape: &'a [usize], new_strides: &'a [usize]) -> Option<Self> {
        if Self::numel(new_shape) != Self::numel(self.shape) {
            return None;
        }
        if new_shape.len() != new_strides.len() {
            return None;
        }

        Some(TensorView{
            data: self.data,
            shape: new_shape,
            strides: new_strides,
    })
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


    #[test]
    fn reshape_changes_rank_not_data() {
        let buf = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let shape = [2, 3];
        let strides = [3, 1];
        let view = TensorView {
            data: &buf,
            shape: &shape,
            strides: &strides,
        };
        let new_shape = [3, 2];
        let new_strides = [2, 1];
        let r = view.reshape(&new_shape, &new_strides).unwrap();

        assert_eq!(r.get(&[0, 1]), Some(2.0));
        assert_eq!(r.get(&[2, 0]), Some(5.0));
        
    }

    #[test]
    fn reshape_rejects_bad_numel() {
        let buf = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let shape = [2, 3];
        let strides = [3, 1];
        let view = TensorView { data: &buf, shape: &shape, strides: &strides };

        let bad_shape = [2, 4];
        let bad_strides = [4, 1];

        assert!(view.reshape(&bad_shape, &bad_strides).is_none());
    }
}
