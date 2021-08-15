use crate::type_utils::FloatScalar;
use ndarray::{Array, Array2, aview1};

pub trait Hasher<T, U> {
    // used Vec to returnd the sized type
    fn to_hash(&self, input: &[T]) -> Vec<U>;
}

#[derive(Debug)]
pub struct RandomProjection<T: FloatScalar> {
    rand_mat: Array2<T>
}

impl RandomProjection<f32> {
    pub fn new(src_dim: usize, trg_dim: usize) -> Self {
        RandomProjection {
            // TODO
            rand_mat: Array::zeros((src_dim, trg_dim))
        }
    }
}

impl Hasher<f32, f32> for RandomProjection<f32> {
    fn to_hash(&self, input: &[f32]) -> Vec<f32> {
        aview1(input).dot(&self.rand_mat).to_vec()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rp() {
        let rp = RandomProjection::new(5, 3);
        let v = vec![1.,2.,3.,4.,5.];
        let hashed_v = rp.to_hash(&v);
        assert_eq!(hashed_v.len(), 3);
    }
}