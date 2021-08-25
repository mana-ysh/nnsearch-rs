// Implementations of hashers are based on `lsh-rs` crate
// https://github.com/ritchie46/lsh-rs/blob/9e81c018872868b319e5fe4d23495ee031117e91/lsh-rs/src/hash.rs#L1
use crate::type_utils::{FloatScalar, SetItem};
use crate::linalg::utils::get_rng;
use ndarray::prelude::*;
use ndarray::{Array, Array2, aview1};
use ndarray_rand::rand_distr::StandardNormal;
use ndarray_rand::RandomExt;
use num::Zero;

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
        let mut rng = get_rng(46);
        RandomProjection {
            rand_mat: Array::random_using((src_dim, trg_dim), StandardNormal, &mut rng)
        }
    }
}

impl Hasher<f32, f32> for RandomProjection<f32> {
    fn to_hash(&self, input: &[f32]) -> Vec<f32> {
        aview1(input).dot(&self.rand_mat).to_vec()
    }
}

#[derive(Debug)]
pub struct MinHash {
    pi_mat: Array2<i32>,
    k: usize,
    dim: usize,
}

impl MinHash {
    pub fn new(k: usize, dim: usize) -> Self {
        let mut pi_mat = Array::zeros((k, dim));
        let mut rng = get_rng(46);
        for row in 0..k {
            let permutation_idx = rand::seq::index::sample(&mut rng, dim, dim)
                .into_iter()
                .map(|idx| (idx + 1) as i32)
                .collect::<Vec<_>>();
            let mut slice = pi_mat.slice_mut(s![row, ..]);
            slice += &aview1(&permutation_idx);
        }
        MinHash {
            pi_mat: pi_mat,
            k: k,
            dim: dim,
        }
    }
}

impl Hasher<SetItem, SetItem> for MinHash {
    fn to_hash(&self, input: &[SetItem]) -> Vec<SetItem> {
        let mut vec: Array1<i32> = Array::zeros(self.dim);
        for idx in 0..input.len() {
            vec[input[idx]] = 1;
        }
        let permutated = &self.pi_mat * &vec;
        let hash = permutated.map_axis(Axis(1), |view| {
            view.into_iter().fold(self.dim, |acc, v| {
                if *v > Zero::zero() {
                    let v = v.clone() as SetItem;
                    if v < acc {
                        v
                    } else {
                        acc
                    }
                } else {
                    acc
                }
            })
        });
        hash.to_vec()
    }
}

#[derive(Debug)]
pub struct BBitMinHash {
}

impl Hasher<i32, bool> for BBitMinHash {
    fn to_hash(&self, input: &[i32]) -> Vec<bool> {
        // TODO
        vec![input[0] == 0]
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

    #[test]
    fn test_minhash() {
        let k = 3;
        let dim =5;
        let minhash = MinHash::new(k, dim);
        let v1 = vec![1, 2, 4];
        let hashed_v1 = minhash.to_hash(&v1);
        assert_eq!(hashed_v1.len(), k);
    }

    #[test]
    #[ignore]
    fn test_approx_jaccard() {
        let large_k = 3000;
        let dim =5;
        let minhash = MinHash::new(large_k, dim);
        let v1 = vec![1, 2, 4];
        let v2 = vec![1, 3];
        let hashed_v1 = minhash.to_hash(&v1);
        let hashed_v2 = minhash.to_hash(&v2);
        assert_eq!(hashed_v1.len(), large_k);
        assert_eq!(hashed_v2.len(), large_k);

        // test the degree of the approximation jaccard
        let expected_jaccard = 1.0 / 4.0;
        let num_match = (0..hashed_v1.len())
            .fold(0, |acc, idx| {
                if hashed_v1[idx] == hashed_v2[idx] {
                    acc + 1
                } else {
                    acc
                }
            });
        assert_eq!((num_match as f32) / (hashed_v1.len() as f32) , expected_jaccard);
    }
}