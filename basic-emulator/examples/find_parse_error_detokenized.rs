use basic_emulator::parser::Parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <program.bas>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("Error reading file: {}", e);
        std::process::exit(1);
    });

    // Parse line by line to find the error
    for (line_idx, line) in source.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        match Parser::parse_program(line) {
            Ok(_) => {
                // Line parsed successfully
            }
            Err(e) => {
                eprintln!("✗ Parse error at line {} (file line {}):", line_idx + 1, line_idx + 1);
                eprintln!("  Error: {}", e);
                eprintln!("  Line content: {}", line);
                std::process::exit(1);
            }
        }
    }

    println!("✓ All lines parsed successfully!");
}
