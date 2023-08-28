use std::io::Write;

use flate2::write::ZlibEncoder;
use flate2::Compression;

use clap::Parser;

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

#[derive(Parser, Debug)]
struct Opts {
    /// First file to compare
    file_a: String,
    /// Second file to compare
    file_b: String,
}
fn main() {
    let input: Vec<_> = std::fs::read_dir("./sites/headers")
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap().path().display().to_string())
        .collect();
    println!("{:?}", input);

    let distance_table: Vec::<Vec<f64>> = Vec::new();

    // transform this into a map(f) function
    for (idx, af) in input.iter().enumerate() {
        println!("\n============={}===============\n", af);
        let ds: Vec<_> = input
            .iter()
            .map(|bf| {
                let a = std::fs::read(af).unwrap();
                let b = std::fs::read(bf).unwrap();

                let d = distance(&b, &a);
                d
            })
            .collect();
        let mut min: Vec<_> = ds
            .iter()
            .enumerate()
            .filter(|(i, n)| *i != idx)
            .map(|(i, x)| (&input[i], *x))
            .collect();

        min.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for (i, (site, score)) in min.iter().enumerate() {

            // for now this needs some manual adjusting, but seems ok.
            if *score < 2.55f64 {
                println!("{} - {} ===> {}", i, site, score);
            }
        }
        //println!("{:?}", min);
    }

    //println!("{}", d);
}
