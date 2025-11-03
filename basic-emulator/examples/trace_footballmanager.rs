use basic_emulator::interpreter::Interpreter;
use basic_emulator::screen::Screen;
use std::fs;

fn main() {
    let source = fs::read_to_string("/Users/enzo/Code/football-manager-ita/footballmanager.bas")
        .expect("Failed to read footballmanager.bas");

    let screen = Screen::new();
    let mut interpreter = Interpreter::new(screen.clone());

    println!("Loading and running footballmanager.bas with detailed trace...\n");

    if let Err(e) = interpreter.load_program(&source) {
        eprintln!("Parse error: {}", e);
        return;
    }

    println!("Program loaded. Executing...\n");

    let mut step_count = 0;
    let max_steps = 200;

    loop {
        if step_count >= max_steps {
            println!("\n=== Stopped after {} steps ===", max_steps);
            break;
        }

        if interpreter.is_waiting_for_input() {
            println!("\n=== INPUT REQUEST at step {} ===", step_count);
            println!("Screen content:");
            println!("---");
            println!("{}", screen.get_content());
            println!("---");
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
                println!("\n\nProgram ended at step {}", step_count);
                break;
            }
            Err(e) => {
                eprintln!("\n\nERROR at step {}: {}", step_count, e);
                println!("\nScreen content at error:");
                println!("---");
                println!("{}", screen.get_content());
                println!("---");
                return;
            }
        }
    }

    println!("\nFinal screen content:");
    println!("===================================");
    println!("{}", screen.get_content());
    println!("===================================");
}
