
use crate::error::NNSearchError;

#[derive(Debug)]
pub enum DistanceType {
    EUCLIDEAN
}

pub fn calcurate_distance(distance_type: &DistanceType, v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    match distance_type {
        DistanceType::EUCLIDEAN => compute_euclidean_distance(v1, v2).unwrap()
    }
}

pub fn compute_euclidean_distance(p1: &Vec<f32>, p2: &Vec<f32>) -> Result<f32, NNSearchError> {
    if p1.len() != p2.len() {
        return Err(NNSearchError::ValueError(format!("Inconsistent length: {} != {}", p1.len(), p2.len())))
    }
    let mut val = 0.0;
    for i in 0..p1.len() {
        val += (p1[i] - p2[i]).powi(2);
    }
    return Ok(val.sqrt());
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