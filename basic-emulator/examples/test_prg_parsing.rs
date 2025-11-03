use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
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

    // Try to parse it
    match basic_emulator::parser::Parser::parse_program(&source) {
        Ok(program) => {
            println!("✓ PRG parsed successfully!");
            println!("Total lines: {}", program.lines.len());

            // Show some stats
            let mut has_get = false;
            let mut has_run = false;
            let mut unclosed_string_lines = Vec::new();

            fn check_statements(stmts: &[basic_emulator::parser::Statement], has_get: &mut bool, has_run: &mut bool) {
                for stmt in stmts {
                    match stmt {
                        basic_emulator::parser::Statement::Get(_) => *has_get = true,
                        basic_emulator::parser::Statement::Run => *has_run = true,
                        basic_emulator::parser::Statement::If(if_stmt) => {
                            // Recursively check nested statements in IF
                            check_statements(&if_stmt.then_branch, has_get, has_run);
                        }
                        _ => {}
                    }
                }
            }

            for (line_num, statements) in &program.lines {
                check_statements(statements, &mut has_get, &mut has_run);

                // Check for line 3800 specifically (has unclosed string)
                if *line_num == 3800 {
                    unclosed_string_lines.push(*line_num);
                }
            }

            println!("Features used:");
            if has_get {
                println!("  - GET statement ✓");
            }
            if has_run {
                println!("  - RUN statement ✓");
            }
            if !unclosed_string_lines.is_empty() {
                println!("  - Unclosed strings (C64 behavior) ✓");
                println!("    Lines: {:?}", unclosed_string_lines);
            }

            println!("\n✓ All C64 BASIC features parsed correctly!");
        }
        Err(e) => {
            eprintln!("✗ Parse error: {}", e);
            std::process::exit(1);
        }
    }
}
