/// Test PRG execution to find runtime errors
/// Runs without TUI to detect errors

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <prg_file>", args[0]);
        std::process::exit(1);
    }

    // Load and detokenize PRG
    let bytes = match basic_emulator::prg_loader::load_prg_file(&args[1]) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error loading PRG: {}", e);
            std::process::exit(1);
        }
    };

    let source = match basic_emulator::prg_loader::detokenize_program(&bytes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error detokenizing: {}", e);
            std::process::exit(1);
        }
    };

    let screen = basic_emulator::screen::Screen::new();
    let mut interp = basic_emulator::interpreter::Interpreter::new(screen.clone());

    match interp.load_program(&source) {
        Ok(_) => println!("✓ Program loaded successfully"),
        Err(e) => {
            eprintln!("✗ Parse error: {}", e);
            std::process::exit(1);
        }
    }

    println!("Running program (max 10000 steps)...");

    let mut steps = 0;
    let max_steps = 10000;

    loop {
        // Check if waiting for input
        if interp.is_waiting_for_input() {
            println!("\n⚠ Program is waiting for input at step {}", steps);
            println!("This is expected behavior - program needs user interaction.");
            println!("\nCurrent output:");
            println!("{}", screen.get_content());
            println!("\n✓ No runtime errors detected!");
            break;
        }

        match interp.step() {
            Ok(true) => {
                steps += 1;
                if steps >= max_steps {
                    println!("\n⚠ Program did not complete within {} steps", max_steps);
                    println!("Last output:");
                    println!("{}", screen.get_content());
                    break;
                }

                // Print progress every 100 steps
                if steps % 100 == 0 {
                    print!(".");
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                }
            }
            Ok(false) => {
                println!("\n✓ Program completed successfully after {} steps", steps);
                println!("\nFinal output:");
                println!("{}", screen.get_content());
                break;
            }
            Err(e) => {
                eprintln!("\n✗ Runtime error at step {}: {}", steps, e);
                println!("\nOutput before error:");
                println!("{}", screen.get_content());
                std::process::exit(1);
            }
        }
    }
}
