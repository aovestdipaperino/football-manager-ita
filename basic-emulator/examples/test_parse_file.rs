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

    match Parser::parse_program(&source) {
        Ok(program) => {
            println!("✓ Parser succeeded!");
            println!("Lines parsed: {}", program.lines.len());
            let mut sorted_lines: Vec<_> = program.lines.keys().collect();
            sorted_lines.sort();
            for line_num in sorted_lines.iter().take(20) {
                let statements = &program.lines[line_num];
                println!("  Line {}: {} statement(s)", line_num, statements.len());
            }
            if sorted_lines.len() > 20 {
                println!("  ... and {} more lines", sorted_lines.len() - 20);
            }
        }
        Err(e) => {
            eprintln!("✗ Parser failed: {}", e);
            std::process::exit(1);
        }
    }
}
