fn main() {
    // Test what the parser sees after normalization
    let parser = basic_emulator::parser::Parser::new("PRINTRIG$\"HELLO\"");
    println!("Parser created for: PRINTRIG$\"HELLO\"");
    
    // Now test as a full line
    let line = "10 PRINTRIG$\"HELLO\"";
    println!("\nParsing: {}", line);
    match basic_emulator::parser::Parser::parse_program(line) {
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
