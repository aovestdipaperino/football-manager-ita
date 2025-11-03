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
    println!("\n=== STARTING EXECUTION ===\n");

    let mut step_count = 0;
    let max_steps = 500; // Run more steps to see more output

    loop {
        if step_count >= max_steps {
            println!("\n=== Reached {} steps ===", max_steps);
            break;
        }

        // Check for input
        if interpreter.is_waiting_for_input() {
            println!("\n=== Program waiting for input at step {} ===", step_count);
            println!("This is where user interaction would happen.");
            break;
        }

        match interpreter.step() {
            Ok(true) => {
                step_count += 1;
            }
            Ok(false) => {
                println!("\n=== Program ended at step {} ===", step_count);
                break;
            }
            Err(e) => {
                eprintln!("\n=== Runtime error at step {}: {} ===", step_count, e);
                return;
            }
        }
    }

    println!("\nExecution completed successfully!");
    println!("Total steps executed: {}", step_count);
}
