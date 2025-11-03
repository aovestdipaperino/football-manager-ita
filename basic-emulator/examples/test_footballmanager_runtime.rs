use basic_emulator::interpreter::Interpreter;
use basic_emulator::screen::Screen;
use std::fs;

fn main() {
    let source = fs::read_to_string("/Users/enzo/Code/football-manager-ita/footballmanager.bas")
        .expect("Failed to read footballmanager.bas");

    let screen = Screen::new();
    let mut interpreter = Interpreter::new(screen.clone());

    println!("Loading footballmanager.bas...");
    if let Err(e) = interpreter.load_program(&source) {
        eprintln!("Error loading program: {}", e);
        return;
    }

    println!("Program loaded successfully!");
    println!("Running first 100 steps...\n");

    let mut step_count = 0;
    let max_steps = 100;

    loop {
        if step_count >= max_steps {
            println!("\nReached {} steps, stopping...", max_steps);
            break;
        }

        // Skip if waiting for input
        if interpreter.is_waiting_for_input() {
            println!("Program is waiting for input at step {}", step_count);
            break;
        }

        match interpreter.step() {
            Ok(true) => {
                step_count += 1;
                if step_count % 10 == 0 {
                    print!(".");
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                }
            }
            Ok(false) => {
                println!("\n\nProgram ended after {} steps", step_count);
                break;
            }
            Err(e) => {
                eprintln!("\n\nRuntime error at step {}: {}", step_count, e);
                return;
            }
        }
    }

    println!("\nTest completed successfully!");
}
