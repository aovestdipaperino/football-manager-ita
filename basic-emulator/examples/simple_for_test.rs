fn main() {
    let input = "FOR PZ=HZTOHZ+15";
    println!("Original: {}", input);

    // Manually trace through normalization
    let upper = input.to_uppercase();
    println!("Uppercase: {}", upper);

    // Try creating a parser
    let _parser = basic_emulator::parser::Parser::new(input);

    println!("\nNow trying to parse as a full line...");
    let line = format!("10 {}", input);
    match basic_emulator::parser::Parser::parse_program(&line) {
        Ok(program) => {
            println!("✓ Success!");
            for (ln, stmts) in &program.lines {
                println!("Line {}", ln);
                for stmt in stmts {
                    println!("  {:?}", stmt);
                }
            }
        }
        Err(e) => {
            println!("✗ Error: {}", e);
        }
    }
}
