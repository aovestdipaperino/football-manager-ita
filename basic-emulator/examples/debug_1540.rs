fn main() {
    let parser = basic_emulator::parser::Parser::new("IFXZ>8ANDXZ<=2THENKJ=2");
    println!("Parser created, now trying to parse line...");
    
    let line = "1540 IFXZ>8ANDXZ<=2THENKJ=2";
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
