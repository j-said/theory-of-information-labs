use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub fn calculate_entropy(counts: &HashMap<u8, u64>, total: f64) -> f64 {
    if total == 0.0 {
        return 0.0;
    } // edge case: empty file 
    let mut entropy = 0.0;
    for &count in counts.values() {
        let p = count as f64 / total;
        if p > 0.0 {
            entropy -= p * p.log2();
        }
    }
    entropy
}


#[derive(Debug, Eq, PartialEq)]
pub struct HuffNode {
    pub freq: u64,
    pub symbol: Option<u8>,
    pub left: Option<Box<HuffNode>>,
    pub right: Option<Box<HuffNode>>,
}

impl Ord for HuffNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq) // sort from smallest to greatest
    }
}

impl PartialOrd for HuffNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn build_huffman_tree(
    counts: &HashMap<u8, u64>,
) -> (HashMap<u8, String>, Option<Box<HuffNode>>) {
    if counts.is_empty() {
        return (HashMap::new(), None); // Edge case: empty file, return empty codes and no tree
    }

    let mut heap = BinaryHeap::new();
    for (&sym, &freq) in counts {
        heap.push(HuffNode {
            freq,
            symbol: Some(sym),
            left: None,
            right: None,
        });
    }

    // Edge case: file consists only of 1 unique symbol
    if heap.len() == 1 {
        let node = heap.pop().unwrap();
        let mut codes = HashMap::new();
        codes.insert(node.symbol.unwrap(), "0".to_string());
        return (codes, Some(Box::new(node)));
    }

    while heap.len() > 1 {
        let n1 = heap.pop().unwrap();
        let n2 = heap.pop().unwrap();
        heap.push(HuffNode {
            freq: n1.freq + n2.freq,
            symbol: None,
            left: Some(Box::new(n1)),
            right: Some(Box::new(n2)),
        });
    }

    let root = heap.pop().unwrap();
    let mut codes = HashMap::new();
    assign_huff_codes(&root, String::new(), &mut codes);

    (codes, Some(Box::new(root)))
}

fn assign_huff_codes(node: &HuffNode, code: String, codes: &mut HashMap<u8, String>) {
    if let Some(sym) = node.symbol {
        codes.insert(sym, code);
    } else {
        if let Some(ref l) = node.left {
            assign_huff_codes(l, format!("{}0", code), codes);
        }
        if let Some(ref r) = node.right {
            assign_huff_codes(r, format!("{}1", code), codes);
        }
    }
}

pub fn print_huffman_tree(node: &HuffNode, prefix: String, is_left: bool) {
    if let Some(ref right) = node.right {
        print_huffman_tree(
            right,
            format!("{}{}", prefix, if is_left { "│   " } else { "    " }),
            false,
        );
    }

    let connector = if is_left { "└── " } else { "┌── " };

    match node.symbol {
        Some(sym) => {
            if sym.is_ascii_graphic() || sym == b' ' {
                println!("{}{}'{}' ({})", prefix, connector, sym as char, node.freq);
            } else {
                println!("{}{}[{:#04X}] ({})", prefix, connector, sym, node.freq);
            }
        }
        None => println!("{}{}* ({})", prefix, connector, node.freq),
    }

    if let Some(ref left) = node.left {
        print_huffman_tree(
            left,
            format!("{}{}", prefix, if is_left { "    " } else { "│   " }),
            true,
        );
    }
}

pub fn build_shannon_fano(counts: &HashMap<u8, u64>) -> HashMap<u8, String> {
    let mut codes = HashMap::new();
    if counts.is_empty() {
        return codes;
    }

    let mut sorted_symbols: Vec<(u8, u64)> = counts.iter().map(|(&k, &v)| (k, v)).collect();
    sorted_symbols.sort_by(|a, b| b.1.cmp(&a.1)); // Sorting by frequency in descending order

    // Edge case: only 1 unique symbol
    if sorted_symbols.len() == 1 {
        codes.insert(sorted_symbols[0].0, "0".to_string());
        return codes;
    }

    shannon_fano_recursive(&sorted_symbols, String::new(), &mut codes);
    codes
}

fn shannon_fano_recursive(symbols: &[(u8, u64)], prefix: String, codes: &mut HashMap<u8, String>) {
    if symbols.len() == 1 {
        codes.insert(symbols[0].0, prefix);
        return;
    }

    let total: u64 = symbols.iter().map(|&(_, f)| f).sum();
    let mut running_sum = 0;
    let mut min_diff = total;
    let mut split_idx = 0;

    // Split into two groups with approximately equal total frequencies
    for i in 0..symbols.len() - 1 {
        running_sum += symbols[i].1;
        let right_sum = total - running_sum;
        let diff = (running_sum as i64 - right_sum as i64).abs() as u64;

        if diff < min_diff {
            min_diff = diff;
            split_idx = i;
        }
    }

    let (left, right) = symbols.split_at(split_idx + 1);
    shannon_fano_recursive(left, format!("{}0", prefix), codes);
    shannon_fano_recursive(right, format!("{}1", prefix), codes);
}
