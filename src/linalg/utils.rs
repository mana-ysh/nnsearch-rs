
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;


pub fn generate_matrix(num: usize, dim: usize) -> Vec<Vec<f32>> {
    let mut rng = get_rng(46);
    let mut mat = vec![];
    for _i in 0..num {
        let mut vec = vec![];
        for _j in 0..dim {
            vec.push(rng.gen::<f32>());
        }
        mat.push(vec);
    }
    mat
}

pub fn get_rng(seed: u64) -> SmallRng {
    SmallRng::seed_from_u64(seed)
}

pub fn to_lowest_b_bit_vector(value: usize, b: usize) -> Vec<bool> {
    let mut vec = vec![];
    let mut inter_value = value;
    for _ in 0..b {
        if inter_value % 2 == 1 {
            vec.push(true);
        } else {
            vec.push(false);
        }
        inter_value /= 2;
    }
    vec
}


mod tests {
    #[test]
    fn test_generate_matrix() {
        let mat: Vec<Vec<f32>> = super::generate_matrix(5, 10);
        assert_eq!(mat.len(), 5);
        for i in 0..mat.len() {
            assert_eq!(mat.get(i).unwrap().len(), 10)
        }
    }

    #[test]
    fn test_to_lowest_b_bit_vector() {
        assert_eq!(super::to_lowest_b_bit_vector(7, 2), vec![true, true]);
        assert_eq!(super::to_lowest_b_bit_vector(6, 2), vec![false, true]);
        assert_eq!(super::to_lowest_b_bit_vector(8, 2), vec![false, false]);
        assert_eq!(super::to_lowest_b_bit_vector(12, 4), vec![false, false, true, true]);
        assert_eq!(super::to_lowest_b_bit_vector(12, 5), vec![false, false, true, true, false]);
    }

}