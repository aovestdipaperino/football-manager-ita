use std::fs;

fn main() {
    let source = fs::read_to_string("footballmanager.bas")
        .expect("Failed to read footballmanager.bas");

    for (idx, line) in source.lines().enumerate() {
        let line_num = idx + 1;
        if line.trim().is_empty() {
            continue;
        }

        match basic_emulator::parser::Parser::parse_program(line) {
            Ok(_) => {
                // Success - continue
            }
            Err(e) => {
                eprintln!("✗ Parse error on line {}:", line_num);
                eprintln!("  Content: {}", line);
                eprintln!("  Error: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("✓ All lines parsed individually!");
}
