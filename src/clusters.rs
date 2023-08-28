//! This module implements the clusterization algorithm, it doesn't know anything about how the
//! distances were built.

use std::collections::{HashSet, VecDeque};

// used for shuffling starting points in the clusterization
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct UnionSet {
    pub parent: Vec<usize>,
    pub size: Vec<u32>,
}

impl UnionSet {
    pub fn new(n: usize) -> Self {
        UnionSet {
            parent: (0..n).into_iter().collect(),
            size: (0..n).into_iter().map(|_| 1).collect(),
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.find(self.parent[x]);
            self.parent[x]
        }
    }

    pub fn join(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);

        let (a, b) = if self.size[i] > self.size[j] {
            (i, j)
        } else {
            (j, i)
        };

        self.size[a] += self.size[b];
        self.parent[b] = a
    }
}

// Takes an MST graph and returns the clusters by slicing the edges with the value higher than
// max_dist
pub fn clusters(graph: &[Vec<(usize, f64)>], max_dist: f64) -> Vec<usize> {
    let mut edges: Vec<Vec<(usize, f64)>> = graph.iter().map(|v| v.iter().filter_map(|(node, w)| {
        if *w <= max_dist {
            Some((*node, *w))
        } else {
            None
        }
    }).collect()).collect();

    edges.iter_mut().for_each(|edges| {
        edges.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    });

    let mut ds = UnionSet::new(graph.len());

    edges.iter().enumerate().for_each(|(i, edges)| {
        edges.iter().for_each(|(j, _)| {
            ds.join(i, *j);
        })
    });

    return ds.parent;
}

// Takes a matrix of distances returns (N-1 edges) and the whole graph
pub fn build_mst(
    dist: &[Vec<f64>],
    n: usize,
) -> (Vec<(usize, usize, f64)>, Vec<Vec<(usize, f64)>>) {
    let mut edges: Vec<_> = dist
        .into_iter()
        .enumerate()
        .map(|(i, v)| v.into_iter().enumerate().map(move |(j, d)| (i, j, *d)))
        .flatten()
        .collect();

    edges.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());

    let mut ds = UnionSet::new(n);
    let mut graph: Vec<Vec<(usize, f64)>> = (0..n).into_iter().map(|_| Vec::new()).collect();

    let mut edg_subset = Vec::new();

    for (i, j, d) in edges {
        if ds.find(i) != ds.find(j) {
            edg_subset.push((i, j, d));
            graph[i].push((j, d));
            graph[j].push((i, d));
            ds.join(i, j);
        }
    }
    (edg_subset, graph)
}
