use basic_emulator::parser::Parser;
use std::fs;

fn main() {
    let source = fs::read_to_string("/Users/enzo/Code/football-manager-ita/footballmanager.bas")
        .expect("Failed to read footballmanager.bas");

    match Parser::parse_program(&source) {
        Ok(program) => {
            println!("Program has {} lines\n", program.lines.len());
            println!("First 20 line numbers:");
            for (i, line_num) in program.lines.keys().take(20).enumerate() {
                println!("  {}: Line {}", i+1, line_num);
            }

            println!("\nLine 10:");
            if let Some(stmts) = program.lines.get(&10) {
                for stmt in stmts {
                    println!("  {:?}", stmt);
                }
            }

            println!("\nLine 20:");
            if let Some(stmts) = program.lines.get(&20) {
                for stmt in stmts {
                    println!("  {:?}", stmt);
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}
