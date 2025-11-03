use basic_emulator::parser::Parser;

fn main() {
    let source = r#"
10 PRINT "HELLO WORLD"
20 LET A = 5
30 FOR I = 1 TO 10
40 PRINT I
50 NEXT I
60 END
"#;

    match Parser::parse_program(source) {
        Ok(program) => {
            println!("✓ Parser succeeded!");
            println!("Lines parsed: {}", program.lines.len());
            for (line_num, statements) in &program.lines {
                println!("  Line {}: {} statement(s)", line_num, statements.len());
            }
        }
        Err(e) => {
            eprintln!("✗ Parser failed: {}", e);
            std::process::exit(1);
        }
    }
}
