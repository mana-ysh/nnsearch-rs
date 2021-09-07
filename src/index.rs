use std::collections::HashMap;

use crate::linalg::distance::PairwiseDistance;
use crate::graph::{GraphOperator, NavigableSmallWorldGraph, VectorNode};

trait VectorIndexOperator {
    fn add(&mut self, data: Vec<f32>) -> Result<(), ()>;
    fn add_batch(&mut self, data_batch: Vec<Vec<f32>>) -> Result<(), ()> {
        for data in data_batch {
            self.add(data).unwrap();
        }
        Ok(())
    }
    fn search(&self, query: Vec<f32>, k: usize) -> Result<Vec<usize>, ()>;
}

#[derive(Debug)]
pub struct NaiveKnnIndex {
    dim: usize,
    distance: Box<dyn PairwiseDistance<f32, f32>>,
    points: Vec<Vec<f32>>,
}

impl VectorIndexOperator for NaiveKnnIndex {
    fn add(&mut self, data: Vec<f32>) -> Result<(), ()> {
        self.points.push(data);
        Ok(())
    }
    fn search(&self, query: Vec<f32>, k: usize) -> Result<Vec<usize>, ()> {
        let scores = self.points
            .iter()
            .map(|vec| self.distance.compute(&query, &vec).unwrap())
            .collect::<Vec<_>>();
        let mut idx = (0..scores.len()).collect::<Vec<usize>>();
        idx.sort_by(|&i, &j| scores[i].partial_cmp(&scores[j]).unwrap());
        Ok(idx[..k].to_vec())
    }
}

//#[derive(Debug)]
pub struct NSWIndex {
    dim: usize,
    graph: Box<dyn GraphOperator>,
}

impl NSWIndex {
    pub fn new(dim: usize, distance: Box<dyn PairwiseDistance<f32, f32>>, trial: usize, min_degree: usize) -> Self {
        NSWIndex{
            dim,
            graph: Box::new(NavigableSmallWorldGraph{
                trial,
                min_degree,
                id2adjacency_ids: HashMap::new(),
                id2node: HashMap::new(),
                distance,
            }),
        }
    }
}

impl VectorIndexOperator for NSWIndex {
    fn add(&mut self, data: Vec<f32>) -> Result<(), ()> {
        if data.len() != self.dim {
            // TODO
            panic!("invalid dim");
        }
        self.graph.add_node(
            VectorNode{id: self.graph.len(), vec: data}
        )
    }
    fn search(&self, query: Vec<f32>, k: usize) -> Result<Vec<usize>, ()> {
        let knn = self.graph.search_nearest_neighbor(&VectorNode{id: usize::MAX, vec: query}, k);
        Ok(knn)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::distance::Euclidean;

    #[test]
    fn test_naive_index() {
        
        let mut index = NaiveKnnIndex{ dim: 2, distance: Box::new(Euclidean{}), points: vec![] };
        index.add(vec![0.1, 0.2]).unwrap();
        index.add(vec![0.1, 0.1]).unwrap();
        let result = index.search(vec![0.1, 0.1], 2).unwrap();
        assert_eq!(result, vec![1, 0]);
        let result = index.search(vec![1.0, 2.0], 2).unwrap();
        assert_eq!(result, vec![0, 1]);
        let result = index.search(vec![1.0, 2.0], 1).unwrap();
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_nsw_index() {
        // node size < k
        let mut index = NSWIndex::new(2, Box::new(Euclidean{}), 3, 4);
        index.add(vec![0.1, 0.2]).unwrap();
        index.add(vec![0.1, 0.1]).unwrap();
        let result = index.search(vec![0.1, 0.1], 2).unwrap();
        assert_eq!(result, vec![1, 0]);
        let result = index.search(vec![1.0, 2.0], 2).unwrap();
        assert_eq!(result, vec![0, 1]);
        let result = index.search(vec![1.0, 2.0], 1).unwrap();
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_nsw_index_dryrun() {
        // node size > k
        let mut index = NSWIndex::new(2, Box::new(Euclidean{}), 3, 4);
        index.add(vec![0.1, 0.2]).unwrap();
        index.add(vec![0.1, 0.3]).unwrap();
        index.add(vec![0.1, 0.4]).unwrap();
        index.add(vec![0.1, 0.5]).unwrap();
        index.add(vec![0.1, 0.6]).unwrap();
        index.add(vec![0.1, 0.7]).unwrap();
        index.search(vec![0.1, 0.1], 2).unwrap();
    }
}
