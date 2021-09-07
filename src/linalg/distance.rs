use crate::error::NNSearchError;
use std::fmt::{Debug};

/// Type of the distance between two objects.
#[derive(Debug)]
pub enum DistanceType {
    /// Euclidean distance
    EUCLIDEAN
}

pub trait PairwiseDistance<T, U>: Debug {
    fn compute(&self, p1: &[T], p2: &[T]) -> Result<U, NNSearchError> {
        if p1.len() != p2.len() {
            return Err(NNSearchError::ValueError(format!("Inconsistent length: {} != {}", p1.len(), p2.len())))
        }
        Ok(self.compute_innter(p1, p2))
    }
    // NOTE: clients should not call this method directly. Use compute.
    fn compute_innter(&self, p1: &[T], p2: &[T]) -> U;
}

#[derive(Debug)]
pub struct Euclidean;

impl PairwiseDistance<f32, f32> for Euclidean {
    fn compute_innter(&self, p1: &[f32], p2: &[f32]) -> f32 {
        let mut val = 0.0;
        for i in 0..p1.len() {
            val += (p1[i] - p2[i]) * (p1[i] - p2[i]);
        }
        val.sqrt()
    }
}

#[derive(Debug)]
pub struct Hamming;

impl PairwiseDistance<bool, u32> for Hamming {
    fn compute_innter(&self, p1: &[bool], p2: &[bool]) -> u32 {
        (0..p1.len()).fold(0, |dist, idx| {
            if p1[idx] == p2[idx] {
                dist
            } else {
                dist + 1
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_euclidean_distance() {
        let dist = Euclidean{};
        let v1 = vec![0.1, 0.2];
        let v2 = vec![0.3, 0.4];
        // FIXME: approx comparison.
        assert_eq!(dist.compute(&v1, &v2).unwrap(), 0.28284273);
    }

    #[test]
    fn test_value_error_compute_euclidean_distance() {
        let dist = Euclidean{};
        let v1 = vec![0.1, 0.2];
        let v2 = vec![0.3];
        // FIXME: loosen check of error message.
        assert_eq!(dist.compute(&v1, &v2).unwrap_err(), NNSearchError::ValueError("Inconsistent length: 2 != 1".to_string()));
    }

    #[test]
    fn test_compute_hamming_distance() {
        let dist = Hamming{};
        // 01001101
        let v1 = vec![false, true, false, false, true, true, false, true];
        // 10111100
        let v2 = vec![true, false, true, true, true, true, false, false];
        assert_eq!(dist.compute(&v1, &v2).unwrap(), 5);
    }
}