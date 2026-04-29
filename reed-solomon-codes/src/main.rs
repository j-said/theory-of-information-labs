use clap::Parser;
use reed_solomon_codes::ReedSolomon;
use std::num::ParseIntError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    clap::ArgGroup::new("mode")
        .required(true)
        .args(["enc", "dec"]),
))]
struct Args {
    #[arg(long = "enc")]
    enc: Option<String>,

    #[arg(long = "dec")]
    dec: Option<String>,

    #[arg(short, long, default_value_t = 6)]
    ecc: usize,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn parse_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    let clean_s = s.replace(" ", "");
    (0..clean_s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&clean_s[i..i + 2], 16))
        .collect()
}
fn main() {
    let args = Args::parse();
    let rs = ReedSolomon::new(args.ecc);

    if let Some(data) = args.enc {
        let data_bytes = data.as_bytes();
        let codeword = rs.encode(data_bytes, args.verbose);

        println!("--- Encoding Mode ---");
        println!("Input Data:       {}", data);

        let hex_encoded: Vec<String> = codeword.iter().map(|b| format!("{:02X}", b)).collect();
        println!("Encoded Codeword: {}", hex_encoded.join(" "));
    } else if let Some(hex_str) = args.dec {
        println!("--- Decoding Mode ---");

        let mut codeword = match parse_hex(&hex_str) {
            Ok(bytes) => bytes,
            Err(_) => {
                eprintln!("Error: Invalid hexadecimal string provided.");
                return;
            }
        };

        println!("Received Hex:     {}", hex_str);

        match rs.correct_errors(&mut codeword, args.verbose) {
            Ok(_) => {
                let data_len = codeword.len() - args.ecc;
                let original_data = &codeword[..data_len];
                let corrected_hex: Vec<String> =
                    codeword.iter().map(|b| format!("{:02X}", b)).collect();

                println!("Status:           SUCCESS (Errors corrected or clean)");
                println!("Cleaned Hex:      {}", corrected_hex.join(" "));
                println!(
                    "Restored Data:    {}",
                    String::from_utf8_lossy(original_data)
                );
            }
            Err(e) => {
                println!("Status:           FAILED TO RESTORE");
                println!("Error Details:    {}", e);
            }
        }
    }
}
