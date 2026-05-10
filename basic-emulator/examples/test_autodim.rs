fn main() {
    let program = r#"
10 REM Test auto-dimensioning
20 SR$(4)="C2":SR$(3)="C1":SR$(2)="B":SR$(1)="A"
30 PRINT SR$(1)
40 PRINT SR$(2)
50 PRINT SR$(3)
60 PRINT SR$(4)
"#;

    let screen = basic_emulator::screen::Screen::new();
    let mut interp = basic_emulator::interpreter::Interpreter::new(screen.clone());

    match interp.load_program(program) {
        Ok(_) => {}
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

    let output = screen.get_content();
    println!("Output:\n{}", output);
    println!("\n✓ Auto-dimensioning test passed!");
}
