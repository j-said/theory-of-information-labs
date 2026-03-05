use calculate_image_enthropy::*;
use image;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args
        .get(1)
        .expect("Usage: cargo run -- <path> [block_size]");
    let block_size: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);

    let img = image::open(file_path)
        .expect("Failed to open image")
        .to_luma8();
    let chunks: Vec<Vec<u8>> = img
        .as_raw()
        .chunks(block_size)
        .map(|c| c.to_vec())
        .collect();
    let total_blocks = chunks.len() as f64;

    let mut counts = HashMap::new();
    for chunk in &chunks {
        *counts.entry(chunk.clone()).or_insert(0) += 1;
    }

    let mut entropy = 0.0;
    let mut probs = HashMap::new();
    for (block, &count) in &counts {
        let p = count as f64 / total_blocks;
        let info = -p.log2();
        entropy += p * info;
        probs.insert(block.clone(), (p, info));
    }

    let huffman_codes = build_huffman_tree(&counts);
    let l_avg: f64 = huffman_codes
        .iter()
        .map(|(block, code)| (counts[block] as f64 / total_blocks) * code.len() as f64)
        .sum();

    println!("\n--- Lab: Compression ---");
    println!("Block: {} pixels", block_size);
    println!("Enthropy (H/Lm): {:.4} bit/message", entropy);
    println!("Avr. length Hoffman (L): {:.4} bit/message", l_avg);
    println!("Efficientcy (L - Lm): {:.4}", l_avg - entropy);

    let test_len = 5.min(chunks.len());
    let (low, high) = arithmetic_demo(&chunks[..test_len], &probs);
    println!("\n--- Arithmetic (First {} blocks) ---", test_len);
    println!("Range: [{:.12}, {:.12})", low, high);
}
