use image;
use qz_dz::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Missing image path.");
        eprintln!("Usage: cargo run -- <path_to_image>");
        process::exit(1);
    }

    let file_path = &args[1];

    let img = match image::open(file_path) {
        Ok(decoded) => decoded.to_luma8(),
        Err(e) => {
            eprintln!("Error: Failed to open image '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    let (w, h) = img.dimensions();

    println!("FILE: {}", file_path);
    println!("Initial Entropy: {:.4}", calculate_entropy(&img));

    for step in [2, 4] {
        let d_img = discretize(&img, step);
        println!("Entropy (Step {}): {:.4}", step, calculate_entropy(&d_img));
        
        let restored = restore_nearest(&d_img, step, w, h);
        let out_name = format!("restored_s{}_{}", step, file_path.split('/').last().unwrap_or("out.png"));
        if let Err(e) = restored.save(&out_name) {
            eprintln!("Warning: Could not save {}: {}", out_name, e);
        }
    }

    for levels in [8, 16, 64] {
        let q_img = quantize(&img, levels);
        println!("Entropy ({} levels): {:.4}", levels, calculate_entropy(&q_img));
        println!(
            "KL Divergence (Orig || {} levels): {:.4}",
            levels,
            calculate_kl_divergence(&img, &q_img)
        );
    }
    println!("--- END ---");
}