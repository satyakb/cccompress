use crate::BitEncoding;

use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
};

#[derive(Serialize, Deserialize, Debug)]
pub enum HuffmanNode<T> {
    Leaf {
        weight: i64,
        element: T,
    },
    Node {
        weight: i64,
        left: Box<HuffmanNode<T>>,
        right: Box<HuffmanNode<T>>,
    },
}

impl<T> HuffmanNode<T> {
    pub fn new_leaf(element: T, weight: i64) -> Self {
        HuffmanNode::Leaf { weight, element }
    }

    pub fn new_node(left: Box<HuffmanNode<T>>, right: Box<HuffmanNode<T>>) -> Self {
        HuffmanNode::Node {
            weight: left.weight() + right.weight(),
            left,
            right,
        }
    }

    pub fn weight(&self) -> i64 {
        match self {
            HuffmanNode::Leaf { weight, .. } => *weight,
            HuffmanNode::Node { weight, .. } => *weight,
        }
    }
}

impl<T> Ord for HuffmanNode<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let c = self.weight().cmp(&other.weight());
        if c != Ordering::Equal {
            c
        } else {
            match (self, other) {
                (HuffmanNode::Leaf { element, .. }, HuffmanNode::Leaf { element: o_el, .. }) => {
                    element.cmp(o_el)
                }
                (HuffmanNode::Leaf { .. }, _) => Ordering::Less,
                (_, HuffmanNode::Leaf { .. }) => Ordering::Greater,
                (_, _) => c,
            }
        }
    }
}

impl<T> PartialOrd for HuffmanNode<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let c = self.weight().partial_cmp(&other.weight());
        if c != Some(Ordering::Equal) {
            c
        } else {
            match (self, other) {
                (HuffmanNode::Leaf { element, .. }, HuffmanNode::Leaf { element: o_el, .. }) => {
                    element.partial_cmp(o_el)
                }
                (HuffmanNode::Leaf { .. }, _) => Some(Ordering::Less),
                (_, HuffmanNode::Leaf { .. }) => Some(Ordering::Greater),
                (_, _) => c,
            }
        }
    }
}

impl<T> PartialEq for HuffmanNode<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                HuffmanNode::Leaf { weight, element },
                HuffmanNode::Leaf {
                    weight: o_w,
                    element: o_el,
                },
            ) => (weight, element).eq(&(o_w, o_el)),
            (HuffmanNode::Node { weight, .. }, HuffmanNode::Node { weight: o_w, .. }) => {
                weight.eq(o_w)
            }
            (_, _) => false,
        }
    }
}

impl<T> Eq for HuffmanNode<T> where T: PartialEq {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub(crate) struct HuffmanTree<T> {
    root: HuffmanNode<T>,
}

impl<T> HuffmanTree<T>
where
    T: Copy + Ord + std::fmt::Debug + Eq + std::hash::Hash,
{
    pub fn from_element(element: T, weight: i64) -> Self {
        HuffmanTree {
            root: HuffmanNode::new_leaf(element, weight),
        }
    }

    pub fn from_nodes(left: HuffmanTree<T>, right: HuffmanTree<T>) -> Self {
        HuffmanTree {
            root: HuffmanNode::new_node(left.root(), right.root()),
        }
    }

    pub fn from_table(table: HashMap<T, i64>) -> Self {
        let initial_trees = table
            .into_iter()
            .map(|(k, v)| HuffmanTree::from_element(k, v))
            .collect();
        Self::merge_trees(initial_trees)
        // todo!();
    }

    fn merge_trees(trees: Vec<Self>) -> Self {
        // Construct min-heap
        let mut heap: BinaryHeap<Reverse<HuffmanTree<T>>> =
            trees.into_iter().map(Reverse).collect();

        while heap.len() > 1 {
            let tmp1 = heap.pop().unwrap().0;
            let tmp2 = heap.pop().unwrap().0;
            let new_tree = HuffmanTree::from_nodes(tmp1, tmp2);
            heap.push(Reverse(new_tree));
        }
        heap.pop().unwrap().0
    }

    pub fn root(self) -> Box<HuffmanNode<T>> {
        Box::new(self.root)
    }

    pub fn prefix_code_table(&self) -> HashMap<T, BitEncoding> {
        let mut codes = HashMap::new();
        let mut stack = Vec::new();
        stack.push((&self.root, BitVec::new()));
        while !stack.is_empty() {
            let (node, code) = stack.pop().unwrap();
            match node {
                HuffmanNode::Leaf { weight: _, element } => {
                    codes.insert(*element, code);
                }
                HuffmanNode::Node {
                    weight: _,
                    left,
                    right,
                } => {
                    let mut left_code = code.clone();
                    left_code.push(false);
                    stack.push((left, left_code));

                    let mut right_code = code.clone();
                    right_code.push(true);
                    stack.push((right, right_code));
                }
            }
        }

        codes
    }

    pub fn decode_bits(&self, bits: BitEncoding) -> Vec<T> {
        let mut ret = Vec::new();
        let mut start = &self.root;
        for b in bits.iter() {
            match start {
                HuffmanNode::Node { left, right, .. } => {
                    if *b {
                        start = right;
                    } else {
                        start = left;
                    }
                }
                _ => (),
            }

            match start {
                HuffmanNode::Leaf { element, .. } => {
                    ret.push(*element);
                    start = &self.root;
                }
                _ => (),
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;
    use std::collections::HashMap;

    use super::HuffmanTree;

    #[test]
    fn tree_sort_simple() {
        let freq_table = HashMap::from([('c', 3), ('b', 2), ('a', 1)]);
        let mut trees: Vec<HuffmanTree<char>> = freq_table
            .into_iter()
            .map(|(k, v)| HuffmanTree::from_element(k, v))
            .collect();
        trees.sort();

        let expected = vec![
            HuffmanTree::from_element('a', 1),
            HuffmanTree::from_element('b', 2),
            HuffmanTree::from_element('c', 3),
        ];
        assert_eq!(trees, expected);
    }

    #[test]
    fn build_tree_simple() {
        let freq_table = HashMap::from([('c', 3), ('b', 2), ('a', 1)]);
        let final_tree = HuffmanTree::from_table(freq_table);

        /*
               6
              / \
             3  c
            / \
           a  b
        */
        let expected = HuffmanTree::from_nodes(
            HuffmanTree::from_nodes(
                HuffmanTree::from_element('a', 1),
                HuffmanTree::from_element('b', 2),
            ),
            HuffmanTree::from_element('c', 3),
        );
        assert_eq!(final_tree, expected);
    }

    #[test]
    fn build_tree_complex() {
        let freq_table = HashMap::from([
            ('c', 32),
            ('d', 42),
            ('e', 120),
            ('k', 7),
            ('l', 42),
            ('m', 24),
            ('u', 37),
            ('z', 2),
        ]);
        let final_tree = HuffmanTree::from_table(freq_table);
        let expected = HuffmanTree::from_nodes(
            // 306
            HuffmanTree::from_element('e', 120), // e,120
            HuffmanTree::from_nodes(
                // 186
                HuffmanTree::from_nodes(
                    // 79
                    HuffmanTree::from_element('u', 37),
                    HuffmanTree::from_element('d', 42),
                ),
                HuffmanTree::from_nodes(
                    // 170
                    HuffmanTree::from_element('l', 42),
                    HuffmanTree::from_nodes(
                        // 65
                        HuffmanTree::from_element('c', 32),
                        HuffmanTree::from_nodes(
                            // 33
                            HuffmanTree::from_nodes(
                                // 9
                                HuffmanTree::from_element('z', 2),
                                HuffmanTree::from_element('k', 7),
                            ),
                            HuffmanTree::from_element('m', 24),
                        ),
                    ),
                ),
            ),
        );
        assert_eq!(final_tree, expected);

        let codes = final_tree.prefix_code_table();
        let expected = HashMap::from([
            ('c', bitvec![1, 1, 1, 0]),
            ('d', bitvec![1, 0, 1]),
            ('e', bitvec![0]),
            ('k', bitvec![1, 1, 1, 1, 0, 1]),
            ('l', bitvec![1, 1, 0]),
            ('m', bitvec![1, 1, 1, 1, 1]),
            ('u', bitvec![1, 0, 0]),
            ('z', bitvec![1, 1, 1, 1, 0, 0]),
        ]);
        assert_eq!(codes, expected);
    }
}
