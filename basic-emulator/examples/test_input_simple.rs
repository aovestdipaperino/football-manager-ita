use basic_emulator::interpreter::Interpreter;
use basic_emulator::screen::Screen;

fn main() {
    let source = r#"
10 PRINT "HELLO"
20 INPUT "WHAT IS YOUR NAME"; N$
30 PRINT "HELLO "; N$
40 END
"#;

    let screen = Screen::new();
    let mut interpreter = Interpreter::new(screen.clone());

    if let Err(e) = interpreter.load_program(&source) {
        eprintln!("Parse error: {}", e);
        return;
    }

    println!("Running program...\n");

    let mut step_count = 0;
    let max_steps = 100;

    loop {
        if step_count >= max_steps {
            println!("\n=== Stopped after {} steps ===", max_steps);
            break;
        }

        if interpreter.is_waiting_for_input() {
            println!("\n=== Waiting for input at step {} ===", step_count);
            println!("Screen content:");
            println!("{}", screen.get_content());

            // Simulate providing input
            println!("\nProviding input: 'ENZO'");
            interpreter.provide_input("ENZO");
            println!("Continuing...\n");
        }

        match interpreter.step() {
            Ok(true) => {
                step_count += 1;
            }
            Ok(false) => {
                println!("\nProgram ended at step {}", step_count);
                break;
            }
            Err(e) => {
                eprintln!("\nERROR at step {}: {}", step_count, e);
                println!("\nScreen at error:");
                println!("{}", screen.get_content());
                return;
            }
        }
    }

    println!("\nFinal screen content:");
    println!("===================================");
    let content = screen.get_content();
    for (i, line) in content.lines().take(10).enumerate() {
        println!("{:02}: |{}|", i, line);
    }
    println!("===================================");
}
