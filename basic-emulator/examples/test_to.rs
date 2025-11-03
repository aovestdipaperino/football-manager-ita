fn main() {
    let tests = vec![
        ("FOR PZ=HZTOHZ+15", "Should normalize TO"),
        ("GOTO100", "Should normalize GOTO"),
    ];

    for (input, desc) in tests {
        println!("\n{}", desc);
        println!("Input: {}", input);

        let line = format!("10 {}", input);
        match basic_emulator::parser::Parser::parse_program(&line) {
            Ok(program) => {
                println!("✓ Parsed successfully");
                for (_, stmts) in &program.lines {
                    for stmt in stmts {
                        println!("  {:?}", stmt);
                    }
                }
            }
            Err(e) => {
                println!("✗ Parse error: {}", e);
            }
        }
    }
}
