//! Entry point: load a petcat-format BASIC source file, spin up the
//! interpreter, and drive a crossterm-based event loop that alternates
//! between interpreter slices and screen redraws/keyboard polls.

mod interp;
mod lang;
mod petscii;
mod screen;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, terminal,
};

use interp::Interp;
use screen::Screen;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: c64basic <file.txt>");
        std::process::exit(2);
    });

    let src = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {}: {}", path, e);
        std::process::exit(1);
    });

    let prog = match lang::load_program(&src) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("parse error: {}", e);
            std::process::exit(1);
        }
    };

    if std::env::args().any(|a| a == "--parse-only") {
        println!("OK: {} lines parsed.", prog.len());
        return;
    }

    if let Some(n) = std::env::args()
        .position(|a| a == "--headless")
        .and_then(|i| std::env::args().nth(i + 1))
        .and_then(|s| s.parse::<u32>().ok())
    {
        let screen = Screen::new();
        let mut interp = Interp::new(prog, screen).unwrap();
        match interp.run_slice(n) {
            Ok(halted) => println!("=== screen after {} steps (halted={}) ===", n, halted),
            Err(e) => println!("=== runtime error: {} ===", e),
        }
        println!(
            "input_mode: {}",
            match interp.input_mode {
                interp::InputMode::Normal => "Normal".to_string(),
                interp::InputMode::AwaitingLine { ref targets, .. } =>
                    format!("AwaitingLine (targets={:?})", targets.len()),
            }
        );
        dump_screen(&interp.screen);
        return;
    }

    let screen = Screen::new();
    let interp = match Interp::new(prog, screen) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = run(interp) {
        // Try to restore the terminal before printing.
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), terminal::LeaveAlternateScreen);
        eprintln!("\nruntime error: {}", e);
        std::process::exit(1);
    }
}

fn run(mut interp: Interp) -> Result<(), String> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().map_err(|e| e.to_string())?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )
    .map_err(|e| e.to_string())?;

    let result = run_loop(&mut interp, &mut stdout);

    let _ = terminal::disable_raw_mode();
    let _ = execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    );

    result
}

fn run_loop(interp: &mut Interp, stdout: &mut io::Stdout) -> Result<(), String> {
    interp.screen.mark_dirty();
    interp
        .screen
        .render(stdout, interp.current_line())
        .map_err(|e| e.to_string())?;

    let mut last_render = Instant::now();
    loop {
        // Poll keyboard with a very short timeout so spin-wait GETs don't
        // peg the CPU.
        if event::poll(Duration::from_millis(2)).map_err(|e| e.to_string())? {
            match event::read().map_err(|e| e.to_string())? {
                Event::Key(ke) if ke.kind != KeyEventKind::Release => {
                    if ke.modifiers.contains(KeyModifiers::CONTROL)
                        && matches!(ke.code, KeyCode::Char('c') | KeyCode::Char('C'))
                    {
                        return Ok(());
                    }
                    for b in key_to_petscii(ke.code) {
                        interp.push_char(b);
                    }
                }
                Event::Resize(_, _) => interp.screen.mark_dirty(),
                _ => {}
            }
        }

        // Execute a slice of the program.
        let budget = if matches!(interp.input_mode, interp::InputMode::Normal) {
            5000
        } else {
            0
        };
        if budget > 0 {
            let halted = interp.run_slice(budget).map_err(|e| e.to_string())?;
            if halted {
                break;
            }
        }

        // Render at ~50 Hz.
        if last_render.elapsed() >= Duration::from_millis(20) {
            interp.screen.mark_dirty();
            interp
                .screen
                .render(stdout, interp.current_line())
                .map_err(|e| e.to_string())?;
            last_render = Instant::now();
        }
    }

    // Final render so the last screen stays visible briefly after halt.
    interp.screen.mark_dirty();
    interp
        .screen
        .render(stdout, interp.current_line())
        .map_err(|e| e.to_string())?;
    std::thread::sleep(Duration::from_millis(300));
    Ok(())
}

fn dump_screen(s: &Screen) {
    for row in &s.cells {
        let line: String = row
            .iter()
            .map(|c| petscii::glyph(c.byte, s.charset))
            .collect();
        println!("|{}|", line);
    }
    println!(
        "(cursor at {}, {}; charset={:?}, color={}, bg={}, border={})",
        s.row, s.col, s.charset, s.color, s.bg, s.border
    );
}

fn key_to_petscii(code: KeyCode) -> Vec<u8> {
    match code {
        KeyCode::Char(c) => {
            // Map ASCII → PETSCII: PETSCII swaps case. Terminal produces
            // lowercase letters by default; C64 expects uppercase in the
            // default (upper/graphics) charset. We send the uppercase form
            // of letters and leave everything else alone.
            let b = if c.is_ascii_lowercase() {
                c.to_ascii_uppercase() as u8
            } else {
                c as u32 as u8
            };
            vec![b]
        }
        KeyCode::Enter => vec![0x0D],
        KeyCode::Backspace => vec![0x14],
        KeyCode::Up => vec![0x91],
        KeyCode::Down => vec![0x11],
        KeyCode::Left => vec![0x9D],
        KeyCode::Right => vec![0x1D],
        KeyCode::Home => vec![0x13],
        KeyCode::Delete => vec![0x14],
        KeyCode::Tab => vec![0x09],
        KeyCode::Esc => vec![0x03],
        _ => vec![],
    }
}
