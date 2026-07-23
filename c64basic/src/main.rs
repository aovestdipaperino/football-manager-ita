//! Entry point: load a petcat-format BASIC source file, spin up the
//! interpreter, and drive a crossterm-based event loop that alternates
//! between interpreter slices and screen redraws/keyboard polls.

use c64basic::{interp, lang, petscii, screen};

use std::io;
use std::io::Read as _;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, terminal,
};

use interp::Interp;
use screen::Screen;

/// Approximate speed of C64 BASIC V2: an empty FOR/NEXT loop runs at
/// roughly this many iterations (statements) per second on real hardware.
const C64_STMTS_PER_SEC: f64 = 600.0;

fn main() {
    let usage = "usage: c64basic <file.txt> [--speed <mult>|max] [--keyport <port>] [--seed <n>] [--parse-only] [--headless <n>] [--c64-font]";

    // --speed 1 (default) approximates real C64 pacing so delay loops and
    // animations take authentic time; --speed max runs unthrottled.
    let mut stmts_per_sec: Option<f64> = Some(C64_STMTS_PER_SEC);
    let mut path: Option<String> = None;
    let mut headless: Option<u32> = None;
    let mut parse_only = false;
    let mut keyport: Option<u16> = None;
    let mut c64_font = false;
    let mut seed: Option<u64> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--speed" => {
                let v = args.next().unwrap_or_default();
                if v.eq_ignore_ascii_case("max") {
                    stmts_per_sec = None;
                } else {
                    match v.parse::<f64>() {
                        Ok(m) if m > 0.0 => stmts_per_sec = Some(C64_STMTS_PER_SEC * m),
                        _ => {
                            eprintln!("--speed expects a positive multiplier or 'max'");
                            std::process::exit(2);
                        }
                    }
                }
            }
            "--headless" => {
                headless = args.next().and_then(|s| s.parse().ok());
                if headless.is_none() {
                    eprintln!("--headless expects a step count");
                    std::process::exit(2);
                }
            }
            "--keyport" => {
                keyport = args.next().and_then(|s| s.parse().ok());
                if keyport.is_none() {
                    eprintln!("--keyport expects a TCP port number");
                    std::process::exit(2);
                }
            }
            "--seed" => {
                seed = args.next().and_then(|s| s.parse().ok());
                if seed.is_none() {
                    eprintln!("--seed expects an unsigned integer");
                    std::process::exit(2);
                }
            }
            "--parse-only" => parse_only = true,
            "--c64-font" => c64_font = true,
            _ if path.is_none() => path = Some(a),
            _ => {
                eprintln!("{}", usage);
                std::process::exit(2);
            }
        }
    }
    let path = path.unwrap_or_else(|| {
        eprintln!("{}", usage);
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

    if parse_only {
        println!("OK: {} lines parsed.", prog.len());
        return;
    }

    let make_interp = |prog| match seed {
        Some(s) => Interp::new_seeded(prog, Screen::new(), s),
        None => Interp::new(prog, Screen::new()),
    };

    if let Some(n) = headless {
        let mut interp = make_interp(prog).unwrap();
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

    let mut interp = match make_interp(prog) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    interp.screen.c64_font = c64_font;

    let key_rx = keyport.map(spawn_key_listener);

    if let Err(e) = run(interp, stmts_per_sec, key_rx) {
        // Try to restore the terminal before printing.
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), terminal::LeaveAlternateScreen);
        eprintln!("\nruntime error: {}", e);
        std::process::exit(1);
    }
}

/// Listen on 127.0.0.1:<port> and forward every byte received to the
/// interpreter as a keypress. Bytes are interpreted as PETSCII, except that
/// lowercase ASCII letters are uppercased (as the terminal path does) and
/// LF is translated to Return. One client at a time; clients may reconnect.
fn spawn_key_listener(port: u16) -> mpsc::Receiver<u8> {
    let (tx, rx) = mpsc::channel();
    let listener = std::net::TcpListener::bind(("127.0.0.1", port)).unwrap_or_else(|e| {
        eprintln!("cannot bind key port {}: {}", port, e);
        std::process::exit(1);
    });
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let mut buf = [0u8; 256];
            loop {
                match stream.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        for &b in &buf[..n] {
                            let b = match b {
                                b'\n' => 0x0D,
                                b'a'..=b'z' => b.to_ascii_uppercase(),
                                _ => b,
                            };
                            if tx.send(b).is_err() {
                                return;
                            }
                        }
                    }
                }
            }
        }
    });
    rx
}

fn run(
    mut interp: Interp,
    stmts_per_sec: Option<f64>,
    key_rx: Option<mpsc::Receiver<u8>>,
) -> Result<(), String> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().map_err(|e| e.to_string())?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )
    .map_err(|e| e.to_string())?;

    let result = run_loop(&mut interp, &mut stdout, stmts_per_sec, key_rx);

    let _ = terminal::disable_raw_mode();
    let _ = execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    );

    result
}

fn run_loop(
    interp: &mut Interp,
    stdout: &mut io::Stdout,
    stmts_per_sec: Option<f64>,
    key_rx: Option<mpsc::Receiver<u8>>,
) -> Result<(), String> {
    interp.screen.mark_dirty();
    interp
        .screen
        .render(stdout, interp.current_line())
        .map_err(|e| e.to_string())?;

    let mut last_render = Instant::now();
    // Token bucket for pacing: earns statements at stmts_per_sec, capped at
    // one second of backlog so a stall doesn't cause a burst.
    let mut last_exec = Instant::now();
    let mut earned: f64 = 0.0;
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

        // Drain any remotely injected keypresses.
        if let Some(rx) = &key_rx {
            while let Ok(b) = rx.try_recv() {
                interp.push_char(b);
            }
        }

        // Execute a slice of the program.
        let now = Instant::now();
        let budget = if !matches!(interp.input_mode, interp::InputMode::Normal) {
            earned = 0.0;
            last_exec = now;
            0
        } else if let Some(rate) = stmts_per_sec {
            earned = (earned + now.duration_since(last_exec).as_secs_f64() * rate).min(rate);
            last_exec = now;
            let b = earned as u32;
            earned -= b as f64;
            b
        } else {
            last_exec = now;
            5000
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
