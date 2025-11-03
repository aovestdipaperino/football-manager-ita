use basic_emulator::parser::Parser;

fn main() {
    let line = "2010 PRINT\"  PREMI SPACE \"";
    println!("Testing line: {}", line);
    match Parser::parse_program(line) {
        Ok(program) => {
            if let Some(stmts) = program.lines.get(&2010) {
                for stmt in stmts {
                    println!("{:?}", stmt);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
