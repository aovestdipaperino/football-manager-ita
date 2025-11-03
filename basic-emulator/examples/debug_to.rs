fn main() {
    let inputs = vec![
        "FOR PZ=HZTOHZ+15",
        "FOR I=1TO10",
        "HZTOHZ",
    ];

    for input in inputs {
        println!("\nInput: {}", input);

        // Simulate what normalize_keywords does
        let upper = input.to_uppercase();
        println!("After uppercase: {}", upper);

        // Create a parser to see normalized result
        let parser = basic_emulator::parser::Parser::new(input);

        // Try to parse
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
