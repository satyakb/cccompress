use cccompress::compressor;

use std::{fs, path::PathBuf};

fn read_file(filename: String) -> Vec<u8> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test/");
    d.push(filename);
    fs::read(d).unwrap()
}

#[test]
fn test_simple() {
    let s = "Hello World!";
    let v = s.to_string().into_bytes();
    let compressed = compressor::compress(v.clone());
    let uncompressed = compressor::uncompress(compressed.clone());
    assert_eq!(v, uncompressed);
}

#[test]
fn test_complex() {
    let v = read_file("les_miserables.txt".to_string());
    let compressed = compressor::compress(v.clone());
    let uncompressed = compressor::uncompress(compressed.clone());
    assert_eq!(v, uncompressed);
}
