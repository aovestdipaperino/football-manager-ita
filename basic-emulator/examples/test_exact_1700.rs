fn main() {
    let line = "1700 IF C(UZ)=0THENPRINTRIG$\"NON HAI \"B$(UZ)\" NELLA TUA SQUADRA\":GOTO1430";
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
