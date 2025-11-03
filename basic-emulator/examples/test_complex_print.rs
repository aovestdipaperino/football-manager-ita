fn main() {
    let line = "10 PRINTRIG$\"NON HAI \"B$(UZ)\" NELLA TUA SQUADRA\"";
    println!("Testing: {}", line);
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
