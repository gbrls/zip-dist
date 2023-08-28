# Zip-Dist

Zip-Dist is a library and program that compares binary data using the
compression length as a distance metric. The basic idea is to compare the
lengths of `C(ab)` vs `C(ac)` to determine if a is closer to `b` or `c`.

```rust
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
```

Currently the main application reads all files in a directory (text or binary)
and tries to make clusters of those files by building a MST and visiting that
MST breaking the edges that have a weight that's higher than a threshold.

This is only an approach that I found to work well but are many other ways to
go about this. In the paper that I used as reference and inspiration, k-means
is used to classify data. It's also important to note that this approach is
very simple and agnostic to the type of data that's fed to it.
