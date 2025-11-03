use std::fs;

fn main() {
    let source = fs::read_to_string("footballmanager.bas")
        .expect("Failed to read footballmanager.bas");

    println!("Parsing footballmanager.bas...");
    match basic_emulator::parser::Parser::parse_program(&source) {
        Ok(program) => {
            println!("✓ Parser succeeded!");
            println!("Total lines: {}", program.lines.len());

            // Show first 10 lines
            println!("\nFirst 10 lines:");
            for (line_num, statements) in program.lines.iter().take(10) {
                println!("  Line {}: {} statement(s)", line_num, statements.len());
            }
        }
        Err(e) => {
            eprintln!("✗ Parser failed: {}", e);
            std::process::exit(1);
        }
    }
}
