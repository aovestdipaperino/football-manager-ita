fn main() {
    let program = r#"
10 PRINT "ENTER NUMBER:"
20 INPUT A
30 PRINT "YOU ENTERED: ";A
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

    // Run until waiting for input
    loop {
        if interp.is_waiting_for_input() {
            println!("✓ Program is waiting for input");
            println!("Output so far:");
            println!("{}", screen.get_content());

            // Simulate user entering "42"
            println!("\nSimulating input: 42\n");
            for c in "42".chars() {
                interp.handle_input_char(c);
            }
            interp.handle_input_enter();
            continue;
        }

        match interp.step() {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) => {
                eprintln!("✗ Runtime error: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("✓ Program completed");
    println!("Final output:");
    println!("{}", screen.get_content());
}
