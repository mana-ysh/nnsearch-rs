use crate::type_utils::FloatScalar;
use crate::error::NNSearchError;

/// Type of the distance between two objects.
#[derive(Debug)]
pub enum DistanceType {
    /// Euclidean distance
    EUCLIDEAN
}

pub fn calcurate_distance<T: FloatScalar>(distance_type: &DistanceType, v1: &[T], v2: &[T]) -> T {
    match distance_type {
        DistanceType::EUCLIDEAN => compute_euclidean_distance(v1, v2).unwrap()
    }
}

pub fn compute_euclidean_distance<T: FloatScalar>(p1: &[T], p2: &[T]) -> Result<T, NNSearchError> {
    if p1.len() != p2.len() {
        return Err(NNSearchError::ValueError(format!("Inconsistent length: {} != {}", p1.len(), p2.len())))
    }
    let mut val = T::get_zero();
    for i in 0..p1.len() {
        val = val + (p1[i] - p2[i]) * (p1[i] - p2[i]);
    }
    Ok(val.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_euclidean_distance() {
        let v1 = vec![0.1, 0.2];
        let v2 = vec![0.3, 0.4];
        // FIXME: approx comparison.
        assert_eq!(compute_euclidean_distance(&v1, &v2).unwrap(), 0.28284273);
    }

    #[test]
    fn test_value_error_compute_euclidean_distance() {
        let v1 = vec![0.1, 0.2];
        let v2 = vec![0.3];
        // FIXME: loosen check of error message.
        assert_eq!(compute_euclidean_distance(&v1, &v2).unwrap_err(), NNSearchError::ValueError("Inconsistent length: 2 != 1".to_string()));
    }
}