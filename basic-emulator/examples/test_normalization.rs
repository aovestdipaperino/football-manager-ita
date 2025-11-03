fn main() {
    let input = "PRINTCHR$(142)";
    let normalized = basic_emulator::parser::Parser::new(input);

    // Can't access the normalized string directly, so let's create a test
    println!("Testing normalization...");

    let test_inputs = vec![
        "PRINTCHR$(142)",
        "GOSUB2000",
        "IFGG<>4",
        "FORAPE=1TO16",
    ];

    for test in test_inputs {
        println!("\nInput:  {}", test);
        let line = format!("10 {}", test);
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
