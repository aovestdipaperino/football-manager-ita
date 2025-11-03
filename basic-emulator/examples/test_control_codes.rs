fn main() {
    let program = r#"
10 PRINT "BEFORE CLEAR"
20 PRINT CHR$(19)
30 PRINT "AFTER CLEAR"
40 PRINT "LINE 1"
50 PRINT CHR$(17);CHR$(17)
60 PRINT "LINE 4"
"#;

    let screen = basic_emulator::screen::Screen::new();
    let mut interp = basic_emulator::interpreter::Interpreter::new(screen.clone());

    match interp.load_program(program) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    }

    loop {
        match interp.step() {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) => {
                eprintln!("Runtime error: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("✓ Program completed");
    println!("Final output:");
    println!("{}", screen.get_content());
    println!("\nExpected: AFTER CLEAR / LINE 4 (BEFORE and LINE 1 cleared)");
}
