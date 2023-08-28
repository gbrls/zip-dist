use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;

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

/// - taken from: '“Low-Resource” Text Classification: A Parameter-Free Classification Method with Compressors
/// - source: https://aclanthology.org/2023.findings-acl.426.pdf
pub fn distance(a: &[u8], b: &[u8]) -> f64 {
    let mut ab = Vec::new();
    ab.extend_from_slice(a);
    ab.extend_from_slice(b);

    let la = compressed_bytes(a);
    let lb = compressed_bytes(b);
    let lab = compressed_bytes(&ab);

    ((lab - la.min(lb)) as f64) / ((la.max(lb)) as f64)
}
