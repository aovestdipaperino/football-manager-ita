fn main() {
    let line = r#"170 PRINTCHR$(142):GOSUB2000:PRINT"""#;

    println!("Parsing: {}", line);
    match basic_emulator::parser::Parser::parse_program(line) {
        Ok(program) => {
            println!("✓ Parsed successfully!");
            for (line_num, statements) in &program.lines {
                println!("Line {}: {} statements", line_num, statements.len());
                for (idx, stmt) in statements.iter().enumerate() {
                    println!("  Statement {}: {:?}", idx + 1, stmt);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Parse error: {}", e);
        }
    }
}
