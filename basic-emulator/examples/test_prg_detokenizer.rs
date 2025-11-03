use basic_emulator::prg_loader::{detokenize_program, load_prg_file};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <program.prg>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    // Load PRG file
    let bytes = load_prg_file(filename).unwrap_or_else(|e| {
        eprintln!("Error loading PRG file: {}", e);
        std::process::exit(1);
    });

    // Detokenize
    let source = detokenize_program(&bytes).unwrap_or_else(|e| {
        eprintln!("Error detokenizing: {}", e);
        std::process::exit(1);
    });

    // Print the result
    println!("{}", source);
}
