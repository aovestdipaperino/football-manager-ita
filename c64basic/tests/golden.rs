//! Golden-screen tests: run the original Football Manager listing headlessly
//! and compare the interpreter's 40x25 screen against fixtures captured from
//! the VICE emulator's screen memory ($0400) at the same point in the game.
//!
//! Fixtures are hex dumps of C64 *screen codes* (what the VIC-II reads), so
//! both sides are translated to Unicode through the crate's own PETSCII
//! table before comparison; the glyph mapping stays single-sourced.
//!
//! To regenerate a fixture: run footballmanager.prg in the vice-mcp build of
//! VICE, drive it to the wanted screen, and dump 1000 bytes at $0400 with
//! vice_memory_read (encoding "hex").

use c64basic::interp::{InputMode, Interp};
use c64basic::lang;
use c64basic::petscii::{glyph, Charset};
use c64basic::screen::Screen;

/// Convert a VIC-II screen code to the PETSCII print code our screen buffer
/// stores. The reverse-video bit (0x80) is dropped: fixtures may catch the
/// flashing cursor cell, and we compare text, not attributes.
fn screen_code_to_petscii(sc: u8) -> u8 {
    let sc = sc & 0x7F;
    match sc {
        0x00..=0x1F => sc + 0x40, // @, A-Z, [, £, ], up, left
        0x20..=0x3F => sc,        // space, digits, punctuation
        0x40..=0x5F => sc + 0x80, // shifted-letter graphics -> $C0-$DF
        _ => sc + 0x40,           // CBM graphics -> $A0-$BF
    }
}

fn fixture_text(name: &str) -> Vec<String> {
    let hex = std::fs::read_to_string(format!(
        "{}/tests/fixtures/{}",
        env!("CARGO_MANIFEST_DIR"),
        name
    ))
    .expect("fixture file");
    let hex = hex.trim();
    let bytes: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("hex"))
        .collect();
    assert_eq!(bytes.len(), 1000, "fixture must be a full 40x25 screen");
    bytes
        .chunks(40)
        .map(|row| {
            row.iter()
                .map(|&sc| glyph(screen_code_to_petscii(sc), Charset::UpperGraphics))
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

fn interp_rows(i: &Interp) -> Vec<String> {
    i.screen
        .to_text()
        .lines()
        .map(|l| l.trim_end().to_string())
        .collect()
}

fn assert_screens_match(actual: &[String], expected: &[String], what: &str) {
    let mut diffs = Vec::new();
    for r in 0..25 {
        let a = actual.get(r).map(String::as_str).unwrap_or("");
        let e = expected.get(r).map(String::as_str).unwrap_or("");
        if a != e {
            diffs.push(format!("row {:2}:\n  vice: |{}|\n  rust: |{}|", r, e, a));
        }
    }
    assert!(
        diffs.is_empty(),
        "{}: {} row(s) differ from VICE\n{}",
        what,
        diffs.len(),
        diffs.join("\n")
    );
}

fn load_game() -> Interp {
    let src = std::fs::read_to_string(format!(
        "{}/../footballmanager.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("footballmanager.txt");
    let prog = lang::load_program(&src).expect("parse");
    Interp::new(prog, Screen::new()).expect("interp")
}

#[test]
fn team_selection_screen_matches_vice() {
    let mut i = load_game();
    // Run until the team-number INPUT at line 850 arms.
    for _ in 0..100 {
        i.run_slice(100_000).unwrap();
        if !matches!(i.input_mode, InputMode::Normal) {
            break;
        }
    }
    assert!(
        matches!(i.input_mode, InputMode::AwaitingLine { .. }),
        "expected the team-selection INPUT prompt"
    );
    assert_screens_match(
        &interp_rows(&i),
        &fixture_text("team-selection.hex"),
        "team selection screen",
    );
}

#[test]
fn main_menu_screen_matches_vice() {
    let mut i = load_game();
    for _ in 0..100 {
        i.run_slice(100_000).unwrap();
        if !matches!(i.input_mode, InputMode::Normal) {
            break;
        }
    }
    // Choose team 8 (JUVENTUS), same as the VICE capture session.
    for &b in b"8\r" {
        i.push_char(b);
    }
    // The main menu ends in a GET spin loop; a bounded slice lands there.
    i.run_slice(200_000).unwrap();
    assert_screens_match(
        &interp_rows(&i),
        &fixture_text("main-menu.hex"),
        "main menu screen",
    );
}
