use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub freq: u64,
    pub symbol: Option<Vec<u8>>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq) // Min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn build_huffman_tree(counts: &HashMap<Vec<u8>, u64>) -> HashMap<Vec<u8>, String> {
    let mut heap = BinaryHeap::new();
    for (sym, &freq) in counts {
        heap.push(Node {
            freq,
            symbol: Some(sym.clone()),
            left: None,
            right: None,
        });
    }

    while heap.len() > 1 {
        let n1 = heap.pop().unwrap();
        let n2 = heap.pop().unwrap();
        heap.push(Node {
            freq: n1.freq + n2.freq,
            symbol: None,
            left: Some(Box::new(n1)),
            right: Some(Box::new(n2)),
        });
    }

    let mut codes = HashMap::new();
    if let Some(root) = heap.pop() {
        assign_codes(&root, String::new(), &mut codes);
    }
    codes
}

fn assign_codes(node: &Node, code: String, codes: &mut HashMap<Vec<u8>, String>) {
    if let Some(ref sym) = node.symbol {
        codes.insert(sym.clone(), code);
    } else {
        if let Some(ref l) = node.left {
            assign_codes(l, format!("{}0", code), codes);
        }
        if let Some(ref r) = node.right {
            assign_codes(r, format!("{}1", code), codes);
        }
    }
}

pub fn arithmetic_demo(data: &[Vec<u8>], probs: &HashMap<Vec<u8>, (f64, f64)>) -> (f64, f64) {
    let mut low = 0.0;
    let mut high = 1.0;
    let mut sorted_keys: Vec<_> = probs.keys().collect();
    sorted_keys.sort();

    for block in data {
        let range = high - low;
        let mut p_low = 0.0;
        for key in &sorted_keys {
            let p = probs[*key].0;
            if *key == block {
                high = low + range * (p_low + p);
                low = low + range * p_low;
                break;
            }
            p_low += p;
        }
    }
    (low, high)
}
