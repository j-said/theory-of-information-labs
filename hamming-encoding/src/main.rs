use std::env;
use std::process;
use hamming_encoding::HammingCodec;

enum Command {
    Encode(String),
    Decode(String),
}

impl Command {
    fn parse(args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 2 {
            return Err("Usage: cargo run -- -enc=\"text\" OR cargo run -- -dec=\"binary_string\"");
        }
        
        let flag = &args[1];
        if let Some(input) = flag.strip_prefix("-enc=") {
            Ok(Self::Encode(input.to_string()))
        } else if let Some(input) = flag.strip_prefix("-dec=") {
            Ok(Self::Decode(input.to_string()))
        } else {
            Err("Unknown flag. Use -enc=... or -dec=...")
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = match Command::parse(&args) {
        Ok(cmd) => cmd,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let codec = HammingCodec::new(8);

    match command {
        Command::Encode(text) => {
            let output: Vec<String> = text
                .bytes()
                .map(|b| format!("{:012b}", codec.encode(b as u32)))
                .collect();
            println!("{}", output.join(" "));
        }
        Command::Decode(binary_str) => {
            let mut text_output = String::new();
            for bin_str in binary_str.split_whitespace() {
                match u32::from_str_radix(bin_str, 2) {
                    Ok(encoded) => {
                        let decoded_byte = codec.decode(encoded) as u8;
                        text_output.push(decoded_byte as char);
                    }
                    Err(_) => {
                        eprintln!("Error parsing binary block: {}", bin_str);
                        process::exit(1);
                    }
                }
            }
            println!("{}", text_output);
        }
    }
}