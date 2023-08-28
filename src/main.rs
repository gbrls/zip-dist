use std::{fs::read_to_string, io::Bytes};

use clap::Parser;

use gzip_cmp::*;

#[derive(Parser, Debug)]
struct Opts {
    /// Directory with files to classify
    #[arg(short)]
    dir: String,

    /// Maximum distance between files to group them together approx: [0, 1.5]
    #[arg(short, default_value_t = 0.45f64)]
    max_dist: f64,
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

    let files: Vec<_> = input.iter().map(|s| read_to_string(s).unwrap()).collect();

    let b: Vec<_> = files.iter().map(|f| f.as_bytes()).collect();

    let clusters = build(&b, opts.max_dist);

    for (i, c) in clusters.iter().enumerate() {
        println!("{} => {}", input[i], input[*c]);
    }
}
