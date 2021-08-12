
use rand::Rng;


pub fn generate_matrix(num: usize, dim: usize) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();
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

mod tests {
    use super::*;

    #[test]
    fn test_generate_matrix() {
        let mat: Vec<Vec<f32>> = generate_matrix(5, 10);
        assert_eq!(mat.len(), 5);
        for i in 0..mat.len() {
            assert_eq!(mat.get(i).unwrap().len(), 10)
        }
    }

}