use image::GrayImage;
use std::f64;

pub fn get_probabilities(img: &GrayImage) -> Vec<f64> {
    let mut counts = vec![0.0; 256];
    let total = (img.width() * img.height()) as f64;
    for p in img.pixels() {
        counts[p[0] as usize] += 1.0;
    }
    counts.into_iter().map(|c| c / total).collect()
}

pub fn calculate_entropy(img: &GrayImage) -> f64 {
    get_probabilities(img)
        .into_iter()
        .filter(|&p| p > 0.0)
        .map(|p| -p * p.log2())
        .sum()
}

pub fn calculate_kl_divergence(img_p: &GrayImage, img_q: &GrayImage) -> f64 {
    let p_dist = get_probabilities(img_p);
    let q_dist = get_probabilities(img_q);
    let epsilon = 1e-10; // Prevents division by zero

    p_dist
        .into_iter()
        .zip(q_dist.into_iter())
        .filter(|&(p, _)| p > 0.0)
        .map(|(p, q)| p * (p / (q + epsilon)).log2())
        .sum()
}

pub fn discretize(img: &GrayImage, step: u32) -> GrayImage {
    let w = (img.width() + step - 1) / step;
    let h = (img.height() + step - 1) / step;
    let mut out = GrayImage::new(w, h);

    for (x, y, pixel) in out.enumerate_pixels_mut() {
        if x * step < img.width() && y * step < img.height() {
            *pixel = *img.get_pixel(x * step, y * step);
        }
    }
    out
}

pub fn quantize(img: &GrayImage, levels: u8) -> GrayImage {
    let mut out = img.clone();
    let bin_size = 256.0 / levels as f32;

    for p in out.pixels_mut() {
        let bin = (p[0] as f32 / bin_size).floor();
        p[0] = (bin * bin_size + bin_size / 2.0).min(255.0) as u8;
    }
    out
}

pub fn restore_nearest(img: &GrayImage, step: u32, orig_w: u32, orig_h: u32) -> GrayImage {
    let mut out = GrayImage::new(orig_w, orig_h);
    for (x, y, pixel) in out.enumerate_pixels_mut() {
        *pixel = *img.get_pixel(
            (x / step).min(img.width() - 1),
            (y / step).min(img.height() - 1),
        );
    }
    out
}
