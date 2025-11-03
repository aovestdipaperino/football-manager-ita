use basic_emulator::interpreter::Interpreter;
use basic_emulator::screen::Screen;
use std::fs;

fn main() {
    let source = fs::read_to_string("/Users/enzo/Code/football-manager-ita/footballmanager.bas")
        .expect("Failed to read footballmanager.bas");

    let screen = Screen::new();
    let mut interpreter = Interpreter::new(screen.clone());

    if let Err(e) = interpreter.load_program(&source) {
        eprintln!("Parse error: {}", e);
        return;
    }

    println!("Running first 1000 steps...\n");

    let mut step_count = 0;
    let max_steps = 1000;

    loop {
        if step_count >= max_steps {
            println!("\n=== Stopped after {} steps ===", max_steps);
            break;
        }

        if interpreter.is_waiting_for_input() {
            println!("\n=== INPUT REQUEST at step {} ===", step_count);
            break;
        }

        match interpreter.step() {
            Ok(true) => {
                step_count += 1;
                if step_count % 100 == 0 {
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
                println!("\nScreen at error:");
                println!("{}", screen.get_content());
                return;
            }
        }
    }

    println!("\nScreen content:");
    println!("===================================");
    let content = screen.get_content();
    for (i, line) in content.lines().take(15).enumerate() {
        println!("{:02}: |{}|", i, line);
    }
    println!("===================================");
}
