use crate::frequency_table::FrequencyTable;
use crate::huffman_tree::HuffmanTree;
use crate::BitEncoding;
use bitvec::prelude::*;
use std::collections::HashMap;

use postcard::{from_bytes, to_allocvec};
extern crate alloc;
use alloc::vec::Vec as Allocvec;

fn build_huffman_tree(v: Vec<u8>) -> HuffmanTree<u8> {
    // Construct frequency table
    let freq_table = FrequencyTable::from_vec(v);
    // Create Huffman tree
    HuffmanTree::from_table(freq_table.table)
}

pub fn compress(v: Vec<u8>) -> Vec<u8> {
    let ht = build_huffman_tree(v.clone());
    let codes: HashMap<u8, BitEncoding> = ht.prefix_code_table();

    // Size of compressed text
    let compressed_size = v
        .iter()
        .fold(0, |acc, e| acc + codes.get(&e).unwrap().len());

    let mut compressed: BitEncoding = BitVec::with_capacity(compressed_size);
    for c in v.iter() {
        let mut code = codes.get(&c).unwrap().clone();
        compressed.append(&mut code);
    }
    serialize(compressed, &ht)
}

fn serialize(compressed: BitEncoding, ht: &HuffmanTree<u8>) -> Vec<u8> {
    let mut ret = Vec::new();
    let mut serialized_tree: Allocvec<u8> = to_allocvec(&ht).unwrap();
    let tree_size = serialized_tree.len().to_be_bytes();

    let writeable_compressed: BitVec<u8, Msb0> = compressed.into_iter().collect();

    ret.append(&mut Vec::from(tree_size));
    ret.append(&mut serialized_tree);
    ret.append(&mut Vec::from(writeable_compressed.len().to_be_bytes()));
    ret.append(&mut Vec::from(writeable_compressed.as_raw_slice()));
    ret
}

pub fn uncompress(v: Vec<u8>) -> Vec<u8> {
    let (compressed, ht) = deserialize(v);
    // Decode compressed bytes using Huffman tree
    ht.decode_bits(compressed)
}

fn deserialize(v: Vec<u8>) -> (BitEncoding, HuffmanTree<u8>) {
    // Read size of serialized Huffman tree
    let (tree_size, v) = v.split_at(std::mem::size_of::<usize>());
    let tree_size = usize::from_be_bytes(tree_size.try_into().unwrap());

    // Read serialized Huffman tree
    let (tree_bytes, v) = v.split_at(tree_size);

    // Read size of compressed bytes
    let (compress_text_size, v) = v.split_at(std::mem::size_of::<usize>());
    let compressed_text_size = usize::from_be_bytes(compress_text_size.try_into().unwrap());

    // Read compressed bytes
    let mut compressed_text_bv: BitVec<u8, Msb0> = BitVec::from_slice(v);
    compressed_text_bv.resize(compressed_text_size, false);

    let compressed_text_bv: BitEncoding = compressed_text_bv.into_iter().collect();

    // Deserialize Huffman tree
    let tree: HuffmanTree<u8> = from_bytes(&tree_bytes).unwrap();

    (compressed_text_bv, tree)
}

#[cfg(test)]
mod test {
    use bitvec::prelude::*;

    #[test]
    fn it_works() {
        let msb = bitvec![u8, Msb0; 1, 1, 1, 1, 0, 0, 0, 0];
        let lsb = bitvec![1, 1, 1, 1, 0, 0, 0, 0];
        let msb2: BitVec = msb.iter().collect();
        assert_eq!(msb, lsb);
        assert_eq!(msb2.as_raw_slice(), lsb.as_raw_slice(),);
    }
}
