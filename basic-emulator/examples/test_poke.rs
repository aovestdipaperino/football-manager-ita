fn main() {
    let line = "10 POKE650,127:POKE1690,0";

    println!("Parsing: {}", line);
    match basic_emulator::parser::Parser::parse_program(line) {
        Ok(program) => {
            println!("✓ Parsed successfully!");
            for (line_num, statements) in &program.lines {
                println!("Line {}: {} statements", line_num, statements.len());
                for stmt in statements {
                    println!("  {:?}", stmt);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Parse error: {}", e);
        }
    }
}
