
use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, BTreeSet};

use crate::linalg::distance::{DistanceType, calcurate_distance};

#[derive(Debug)]
pub struct VectorNode {
    pub id: usize,
    pub vec: Vec<f32>,
}


#[derive(Debug)]
struct CostedItem {
    pub id: usize,
    pub cost: f32,
}

impl Ord for CostedItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(&other) {
            Some(order) => order,
            None => {
                panic!("Fail to compare item: self={:?}, other={:?}", self, other);
            }
        }
    }
}

impl PartialOrd for CostedItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl PartialEq for CostedItem {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.id == other.id
    }
}

impl Eq for CostedItem {}

#[derive(Debug)]
struct DistanceCache {
    pub cache: HashMap<usize, f32>
}

impl DistanceCache {
    fn new() -> Self {
        DistanceCache {
            cache: HashMap::new()
        }
    }

    fn get_distance(&mut self, distance_type: &DistanceType, query: &VectorNode, target: &VectorNode) -> f32 {
        if !self.cache.contains_key(&target.id) {
            self.cache.insert(
                target.id,
                calcurate_distance(distance_type, &query.vec, &target.vec)
            );
        } 
        return self.cache.get(&target.id).unwrap().clone()
    }
}

#[derive(Debug)]
pub struct NavigableSmallWorldGraph {
    pub trial: usize,
    pub min_degree: usize,
    pub id2adjacency_ids: HashMap<usize, Vec<usize>>,
    pub id2node: HashMap<usize, VectorNode>,
    pub distance_type: DistanceType,
}


impl NavigableSmallWorldGraph {
    fn approx_knn_search(&self, query: &VectorNode, k: usize) -> Vec<usize> {
        let dist_cache = DistanceCache::new();

        if self.id2node.len() <= k {
            // FIXME: notify that returned result is not satisfied with size k.
            let mut incomplete_result: Vec<usize> = self.id2node.keys().cloned().collect();
            incomplete_result.sort_by(
                |&a, &b|
                calcurate_distance(&self.distance_type, &query.vec, &self.get_node(&a).unwrap().vec)
                    .partial_cmp(&calcurate_distance(&self.distance_type, &query.vec, &self.get_node(&b).unwrap().vec)).unwrap());
            return incomplete_result
        }
        // The algorithm here is based on https://publications.hse.ru/mirror/pubs/share/folder/x5p6h7thif/direct/128296059
        let mut rng = rand::thread_rng();
        let mut candidates: BTreeSet<CostedItem> = BTreeSet::new();
        let mut visited = HashSet::new();
        let mut result: BTreeSet<CostedItem> = BTreeSet::new();
        let mut dist_cache = DistanceCache::new();
        let ids: Vec<&usize> = self.id2node.iter().map(|(k, v)| k).collect();
        for i in 0..self.trial {
            let entry_id = &(*ids.choose(&mut rng).unwrap()).clone();
            candidates.insert(CostedItem {id: *entry_id, cost: dist_cache.get_distance(&self.distance_type, &query, self.get_node(&entry_id).unwrap())});
            let mut temp_res = HashSet::new();
            loop {
                let c = candidates.pop_first();
                if c.is_none() {
                    break
                }
                let c = c.unwrap();
                if result.len() >= k {
                    let kth_dist = dist_cache.get_distance(&self.distance_type, &query, self.get_node(&result.iter().nth(k-1).unwrap().id).unwrap());
                    if kth_dist <= c.cost {
                        break
                    }
                }
                for &id in self.id2adjacency_ids.get(&c.id).unwrap_or(&vec![]) {
                    if !visited.contains(&id) {
                        visited.insert(id.clone());
                        candidates.insert(CostedItem {id: id, cost: dist_cache.get_distance(&self.distance_type, &query, self.get_node(&id).unwrap())});
                        temp_res.insert(id.clone());
                    }
                }
                if !visited.contains(&c.id) {
                    visited.insert(c.id.clone());
                    temp_res.insert(c.id.clone());
                }
                for &id in &temp_res {
                    result.insert(CostedItem {id: id, cost: dist_cache.get_distance(&self.distance_type, &query, self.get_node(&id).unwrap())});
                }
            }
        }
        result.iter().map(|item| item.id).collect::<Vec<_>>()[..k].to_vec()
    }
}


pub trait GraphOperator {
    fn add_node(&mut self, node: VectorNode) -> Result<(), ()>;
    fn get_node(&self, id: &usize) -> Option<&VectorNode>;
    fn search_nearest_neighbor(&self, query: &VectorNode, k: usize) -> Vec<usize>;
    fn len(&self) -> usize;
}

impl GraphOperator for NavigableSmallWorldGraph {
    fn add_node(&mut self, node: VectorNode) -> Result<(), ()> {
        if self.id2node.len() == 0 {
            self.id2node.insert(node.id, node);
            return Ok(())
        }
        // FIXME: handling the case where node.id is duplicated.
        let nn_ids = self.search_nearest_neighbor(&node, self.min_degree);
        // connect node -> nn
        self.id2adjacency_ids.insert(node.id, nn_ids.clone());
        // connect nn -> node
        nn_ids.iter().for_each(|nn_id|
            {
                if !self.id2adjacency_ids.contains_key(nn_id) {
                    self.id2adjacency_ids.insert(nn_id.clone(), vec![]);
                }
                self.id2adjacency_ids.get_mut(nn_id).unwrap().push(node.id.clone())
            }
        );
        self.id2node.insert(node.id, node);
        Ok(())
    }
    fn get_node(&self, id: &usize) -> Option<&VectorNode> {
        return self.id2node.get(id)
    }
    fn search_nearest_neighbor(&self, query: &VectorNode, k: usize) -> Vec<usize> {
        return self.approx_knn_search(query, k);
    }

    fn len(&self) -> usize {
        return self.id2node.len()
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_nsw() {
        // pass 
    }

}
