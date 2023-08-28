use std::collections::{HashSet, VecDeque};
use std::io::Write;

use flate2::write::ZlibEncoder;
use flate2::Compression;

use clap::Parser;

// used for shuffling starting points in the clusterization
use rand::seq::SliceRandom;
use rand::thread_rng;

trait Backend {
    fn compress(d: &[u8]) -> Vec<u8>;
}

struct Gzip {}
impl Backend for Gzip {
    fn compress(d: &[u8]) -> Vec<u8> {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(d).unwrap();
        let compressed_bytes = e.finish().unwrap();
        return compressed_bytes;
    }
}

fn compress<T: Backend>(d: &[u8]) -> u32 {
    T::compress(d).len() as u32
}

fn compressed_bytes(a: &[u8]) -> u32 {
    compress::<Gzip>(a)
}

// - taken from: '“Low-Resource” Text Classification: A Parameter-Free Classification Method with Compressors
// - source: https://aclanthology.org/2023.findings-acl.426.pdf
fn distance(a: &[u8], b: &[u8]) -> f64 {
    let mut ab = Vec::new();
    ab.extend_from_slice(a);
    ab.extend_from_slice(b);

    let la = compressed_bytes(a);
    let lb = compressed_bytes(b);
    let lab = compressed_bytes(&ab);

    ((lab - la.min(lb)) as f64) / ((la.max(lb)) as f64)
}

fn ds_find(i: usize, parent: &mut [usize]) -> usize {
    if parent[i] == i {
        i
    } else {
        parent[i] = ds_find(parent[i], parent);
        parent[i]
    }
}

fn ds_join(i: usize, j: usize, parent: &mut [usize]) {
    let i = ds_find(i, parent);
    let j = ds_find(j, parent);

    let a = i.min(j);
    let b = i.max(j);

    parent[a] = b
}

fn clusters(graph: &[Vec<(usize, f64)>], max_dist: f64) -> Vec<Vec<usize>> {
    let mut rng = thread_rng();
    let mut start: Vec<_> = (0..graph.len()).into_iter().collect();
    start.shuffle(&mut rng);

    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut clusters: Vec<Vec<usize>> = Vec::new();

    for i in start {
        if visited.contains(&i) {
            continue;
        }

        queue.push_back(i);
        clusters.push(vec![i]);

        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            if visited.contains(&u) {
                continue;
            }

            visited.insert(u);

            for (v, d) in &graph[u] {
                if *d < max_dist {
                    queue.push_back(*v);
                    clusters.last_mut().unwrap().push(*v);
                }
            }
        }
    }

    clusters.sort_by_key(|c| c.len());
    clusters
}

fn build_mst(dist: &[Vec<f64>], n: usize) -> (Vec<(usize, usize, f64)>, Vec<Vec<(usize, f64)>>) {
    let mut edges: Vec<_> = dist
        .into_iter()
        .enumerate()
        .map(|(i, v)| v.into_iter().enumerate().map(move |(j, d)| (i, j, *d)))
        .flatten()
        .collect();

    edges.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());

    let mut parent: Vec<usize> = (0..n).into_iter().collect();
    let mut graph: Vec<Vec<(usize, f64)>> = (0..n).into_iter().map(|_| Vec::new()).collect();

    println!("{:#?}", &parent);

    let mut edg_subset = Vec::new();

    for (i, j, d) in edges {
        if ds_find(i, &mut parent) != ds_find(j, &mut parent) {
            edg_subset.push((i, j, d));
            graph[i].push((j, d));
            graph[j].push((i, d));
            ds_join(i, j, &mut parent);
        }
    }
    (edg_subset, graph)
}

#[derive(Parser, Debug)]
struct Opts {
    /// Directory with files to classify
    #[arg(short)]
    dir: String,

    /// Maximum distance between files to group them together (around 0.0 - 1.5)
    #[arg(short, default_value_t = 0.45f64)]
    max_dist: f64
}

fn main() {

    let opts = Opts::parse();

    let input: Vec<_> = std::fs::read_dir(opts.dir)
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| std::fs::metadata(x.path()).unwrap().is_file())
        .map(|x| x.path().display().to_string())
        .collect();
    println!("{:?}", input);

    let distance_table: Vec<Vec<f64>> = input
        .iter()
        .enumerate()
        .map(|(_i, af)| {
            let ds: Vec<f64> = input
                .iter()
                .map(|bf| {
                    let a = std::fs::read(af).unwrap();
                    let b = std::fs::read(bf).unwrap();

                    let d = distance(&b, &a);
                    d
                })
                .collect();
            let min: Vec<_> = ds
                .iter()
                .enumerate()
                //.filter(|(i, n)| *i != idx)
                //.map(|(i, x)| (&input[i], *x))
                .map(|(_j, x)| *x)
                .collect();
            min
        })
        .collect();

    //println!("{:#?}", distance_table);
    let (mst, graph) = build_mst(&distance_table, input.len());
    for (i, j, d) in mst {
        println!("{} {} => {}", input[i], input[j], d);
    }
    let clusters = clusters(&graph, opts.max_dist);

    println!("\n\n=== Clusters ===");
    for c in clusters {
        println!("\n=================\n");
        for i in c {
            println!("{}", input[i]);
        }
    }
}
