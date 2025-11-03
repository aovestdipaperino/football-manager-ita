fn main() {
    let input = "IFQZ<HZORQZ>HZ+15THEN820";
    println!("Input: {}", input);

    let line = format!("1080 QZ=VAL(A$):{}", input);
    println!("\nFull line: {}", line);

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
