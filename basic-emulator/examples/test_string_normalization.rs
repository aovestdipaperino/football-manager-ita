use basic_emulator::parser::Parser;

fn main() {
    let input = "PRINT\"PREMI SPACE\"";
    println!("Input: {}", input);

    // We need to access the normalize function... but it's private
    // Let's just test via parsing
    let test = "10 PRINT\"PREMI SPACE\"";
    match Parser::parse_program(test) {
        Ok(program) => {
            if let Some(stmts) = program.lines.get(&10) {
                println!("Parsed: {:?}", stmts);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    // Test more cases
    let tests = vec![
        "10 PRINT\"PREMERE\"",
        "10 PRINT\"PREMI\"",
        "10 PRINT\"REMOTO\"",
        "10 PRINT\"INFORMAZIONE\"",
    ];

    for test in tests {
        match Parser::parse_program(test) {
            Ok(program) => {
                if let Some(stmts) = program.lines.get(&10) {
                    println!("{} -> {:?}", test, stmts[0]);
                }
            }
            Err(e) => {
                eprintln!("{} -> Error: {}", test, e);
            }
        }
    }
}
