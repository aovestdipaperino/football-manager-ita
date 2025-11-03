mod interpreter;
mod parser;
mod prg_loader;
mod screen;
mod value;

use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, fs, io, process, time::Duration};

use interpreter::Interpreter;
use prg_loader::{detokenize_program, load_prg_file};
use screen::Screen;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let mut prg_mode = false;
    let mut filename = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--prg" => {
                prg_mode = true;
            }
            arg if !arg.starts_with("--") => {
                filename = Some(arg.to_string());
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                print_usage(&args[0]);
                process::exit(1);
            }
        }
        i += 1;
    }

    let filename = filename.unwrap_or_else(|| {
        print_usage(&args[0]);
        process::exit(1);
    });

    // Load the program source
    let source = if prg_mode {
        // Load and detokenize PRG file
        let bytes = load_prg_file(&filename)
            .unwrap_or_else(|e| {
                eprintln!("Error reading PRG file {}: {}", filename, e);
                process::exit(1);
            });

        detokenize_program(&bytes)
            .unwrap_or_else(|e| {
                eprintln!("Error detokenizing PRG file {}: {}", filename, e);
                process::exit(1);
            })
    } else {
        // Load plain text BASIC file
        fs::read_to_string(&filename)
            .unwrap_or_else(|e| {
                eprintln!("Error reading file {}: {}", filename, e);
                process::exit(1);
            })
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut screen = Screen::new();
    let mut interpreter = Interpreter::new(screen.clone());

    // Load and run program
    if let Err(e) = interpreter.load_program(&source) {
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        eprintln!("Error loading program: {}", e);
        process::exit(1);
    }

    let result = run_interpreter(&mut terminal, &mut interpreter, &mut screen);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Runtime error: {}", e);
        process::exit(1);
    }

    Ok(())
}

fn run_interpreter(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    interpreter: &mut Interpreter,
    screen: &mut Screen,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| screen.draw(f))?;

        // Check if program is waiting for input
        if interpreter.is_waiting_for_input() {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        interpreter.handle_input_char(c);
                    }
                    KeyCode::Enter => {
                        interpreter.handle_input_enter();
                    }
                    KeyCode::Backspace => {
                        interpreter.handle_input_backspace();
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        } else {
            // Check for escape key to quit
            if poll(Duration::from_millis(0))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Esc {
                        return Ok(());
                    }
                }
            }

            // Execute next instruction
            match interpreter.step() {
                Ok(true) => {
                    // Add small delay to throttle execution and allow screen updates
                    std::thread::sleep(Duration::from_micros(100));
                }
                Ok(false) => {
                    // Program ended - wait for user to press a key
                    event::read()?;
                    return Ok(());
                }
                Err(e) => {
                    screen.print(&format!("\nERROR: {}", e));
                    terminal.draw(|f| screen.draw(f))?;
                    // Wait for user to press a key before exiting
                    event::read()?;
                    return Err(io::Error::new(io::ErrorKind::Other, e));
                }
            }
        }
    }
}

fn print_usage(program_name: &str) {
    eprintln!("Usage: {} [--prg] <program.bas|program.prg>", program_name);
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --prg    Load a tokenized Commodore 64 PRG file instead of plain text BASIC");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} program.bas           # Load plain text BASIC file", program_name);
    eprintln!("  {} --prg program.prg     # Load tokenized PRG file", program_name);
}
