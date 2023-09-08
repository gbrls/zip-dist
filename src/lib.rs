mod clusters;
mod zip_distance;

use std::vec;

use clusters::*;
use zip_distance::*;

use rayon::prelude::*;

//pub struct Builder {
//    max_dist: f64
//}
//
//impl Builder {
//    pub fn new() -> Self {
//        Builder { max_dist: 0.45 }
//    }
//
//    pub fn distance(mut self, d: f64) -> Self {
//        self.max_dist = d;
//        self
//    }
//
//    pub fn build_groups(self, data: &[&[u8]]) -> Vec<usize> {
//        vec![]
//    }
//}

fn build_distance_table(data: &[&[u8]]) -> Vec<Vec<f64>> {
    data.par_iter()
        .map(|a| data.par_iter().map(|b| distance(a, b)).collect())
        .collect()
}

pub fn build(data: &[&[u8]], max_dist: f64) -> Vec<usize> {
    println!("Building distance table...");
    let d = build_distance_table(data);
    println!("Building mst...");
    let (_mst, graph) = build_mst(&d, data.len());
    println!("Building clusters...");
    return clusters(&graph, max_dist);
}