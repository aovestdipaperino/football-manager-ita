fn main() {
    let line = "1540 IFXZ>8ANDXZ<=2THENKJ=2:GOSUB1220";
    println!("Parsing: {}", line);
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
