fn main() {
    let program = r#"
10 PRINT CHR$(142);"lowercase: abc"
20 PRINT CHR$(14);"UPPERCASE: ABC"
30 PRINT CHR$(142);"mixed: AbC"
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
    println!("Output:");
    println!("{}", screen.get_content());
}
