fn main() {
    let program = r#"
10 PRINT CHR$(5);"WHITE TEXT"
20 PRINT CHR$(28);"RED TEXT"
30 PRINT CHR$(30);"GREEN TEXT"
40 PRINT CHR$(31);"BLUE TEXT"
50 PRINT CHR$(158);"YELLOW TEXT"
60 PRINT CHR$(159);"CYAN TEXT"
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
    println!("Output (colors not visible in plain text):");
    println!("{}", screen.get_content());
    println!("\nNote: Colors are stored and will be displayed in TUI mode");
    println!("WHITE, RED, GREEN, BLUE, YELLOW, CYAN should each be in their color");
}
