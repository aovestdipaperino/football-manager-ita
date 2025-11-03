fn main() {
    let line = "1700 IFC(UZ)=0THENPRINTRIG$\"NON HAI \"B$(UZ)\" NELLA TUA SQUADRA\":GOTO1430";
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
