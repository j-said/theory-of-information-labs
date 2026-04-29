use compression_shannon_fano::*;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <path_to_text_file>");
        return;
    }

    let file_path = &args[1];
    let data = match fs::read(file_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            return;
        }
    };

    let total_symbols = data.len() as f64;

    println!("File: {} ({} bytes)", file_path, data.len());

    if total_symbols == 0.0 {
        println!("File is empty. Entropy: 0, Average length: 0");
        return;
    }

    let mut counts = HashMap::new();
    for &byte in &data {
        *counts.entry(byte).or_insert(0) += 1;
    }

    println!("Unique symbols: {}", counts.len());

    let entropy = calculate_entropy(&counts, total_symbols);

    #[allow(unused_variables)]
    let (huffman_codes, root_node) = build_huffman_tree(&counts);
    let fano_codes = build_shannon_fano(&counts);

    let calc_l = |codes: &HashMap<u8, String>| -> f64 {
        codes
            .iter()
            .map(|(&sym, code)| (counts[&sym] as f64 / total_symbols) * code.len() as f64)
            .sum()
    };

    let l_huffman = calc_l(&huffman_codes);
    let l_fano = calc_l(&fano_codes);

    println!("Entropy (H): {:.4} bit/symbol", entropy);

    // println!("\n--- Huffman Tree ---");
    // if let Some(root) = root_node {
    //     if counts.len() == 1 {
    //         println!("Single symbol file. Node code assigned as '0'.");
    //     } else {
    //         if counts.len() <= 50 {
    //             print_huffman_tree(&root, String::new(), true);
    //         } else {
    //             println!(
    //                 "Tree is too large to display ({} unique symbols).",
    //                 counts.len()
    //             );
    //         }
    //     }
    // }
    
    println!("-----------------------------");
    println!("Huffman Average Length (L): {:.4} bit/symbol", l_huffman);
    println!("Huffman Efficiency (L - H): {:.4}", l_huffman - entropy);
    println!("-----------------------------");
    println!("Shannon-Fano Average Length (L): {:.4} bit/symbol", l_fano);
    println!("Shannon-Fano Efficiency (L - H): {:.4}\n\n", l_fano - entropy);
}
