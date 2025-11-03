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
        eprintln!("Error: {}", e);
        return;
    }

    println!("Executing until first INPUT...\n");

    let mut step_count = 0;
    loop {
        if interpreter.is_waiting_for_input() {
            println!("Program is waiting for input after {} steps\n", step_count);
            break;
        }

        match interpreter.step() {
            Ok(true) => step_count += 1,
            Ok(false) => {
                println!("Program ended after {} steps\n", step_count);
                break;
            }
            Err(e) => {
                eprintln!("Error at step {}: {}", step_count, e);
                return;
            }
        }

        if step_count > 1000 {
            println!("Stopping after 1000 steps");
            break;
        }
    }

    println!("=== SCREEN OUTPUT ===");
    println!("{}", screen.get_content());
    println!("=== END SCREEN ===");
}
