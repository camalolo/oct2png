use base64::Engine;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--help".into()) {
        println!("oct2png - Decode base64-encoded PNG data from stdin and save to a file");
        println!();
        println!("Usage: echo '<data>base64,<base64_data>' | oct2png <output_path>");
        process::exit(0);
    }

    if args.len() != 2 {
        eprintln!("Usage: echo '<data>base64,<base64_data>' | oct2png <output_path>");
        process::exit(1);
    }

    let output_path = &args[1];

    if let Err(e) = run(output_path) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;

    let data = extract_base64_data(&content).ok_or("no base64 marker found in input")?;

    let decoded = base64::engine::general_purpose::STANDARD.decode(data)?;

    fs::write(output_path, decoded)?;

    println!("{output_path}");
    Ok(())
}

fn extract_base64_data(content: &str) -> Option<&str> {
    let marker = "base64,";
    let idx = content.find(marker)?;
    Some(content[idx + marker.len()..].trim())
}
