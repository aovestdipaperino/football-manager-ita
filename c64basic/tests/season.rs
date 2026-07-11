//! Play a whole season of the international edition by reading the screen and
//! pressing the key each prompt asks for, exactly as a human would. This
//! exercises the promotion/relegation and season-end code paths (never run
//! before the bug fixes) and proves the fixed listing survives 15 matches
//! without the crashes documented for the original: the GIR animation hang,
//! the BAD SUBSCRIPT on substitution, etc.
//!
//! Seeded so the run is reproducible.

use c64basic::interp::{InputMode, Interp};
use c64basic::lang;
use c64basic::screen::Screen;

fn load_intl(seed: u64) -> Interp {
    let src = std::fs::read_to_string(format!(
        "{}/../football-manager-intl.bas",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("football-manager-intl.bas");
    let prog = lang::load_program(&src).expect("parse");
    Interp::new_seeded(prog, Screen::new(), seed).expect("interp")
}

/// Decide which key(s) to send given the current screen text. Returns the
/// bytes to push, or None if this looks like the end-of-season prompt.
fn respond(text: &str) -> Option<&'static [u8]> {
    // Only three screens need a key other than SPACE-to-continue.
    if text.contains("TO PLAY THE NEW SEASON") {
        None // reached season end
    } else if text.contains("WHAT IS YOUR OFFER") || text.contains("TRANSFER  MARKET") {
        Some(b"^\r") // transfer market: buy nobody
    } else if text.contains("TO SELL OR LIST PLAYERS") {
        Some(b"G") // main menu: play the match
    } else {
        // Every other screen (ready, line-up, half-time, final score, other
        // results, league table, weekly balance, end of season) is a
        // SPACE-to-continue prompt.
        Some(b" ")
    }
}

#[test]
fn full_season_reaches_promotion() {
    let mut i = load_intl(4242);
    let mut champion_seen = false;
    let mut promo_or_releg_seen = false;

    // Prime: run to the first prompt (team selection).
    for _ in 0..50 {
        i.run_slice(200_000).unwrap();
        if !matches!(i.input_mode, InputMode::Normal) {
            break;
        }
    }
    // Choose team 15 (REAL MADRID after alphabetical sort).
    for &b in b"15\r" {
        i.push_char(b);
    }

    // Screen-driven play. Bounded so a stuck prompt fails loudly instead of
    // hanging (the GIR bug would manifest as a timeout without this cap).
    for _ in 0..600 {
        i.run_slice(400_000).unwrap();
        let text = i.screen.to_text();
        if text.contains("CHAMPION OF EUROPE") {
            champion_seen = true;
        }
        if text.contains("PROMOTION FOR") || text.contains("RELEGATION FOR") {
            promo_or_releg_seen = true;
        }
        match respond(&text) {
            None => {
                // Season complete without a crash. In the bottom division a
                // top-4 finish triggers promotion messaging.
                assert!(
                    promo_or_releg_seen,
                    "expected promotion/relegation messaging at season end"
                );
                let _ = champion_seen; // informational; depends on final place
                return;
            }
            Some(keys) => {
                for &b in keys {
                    i.push_char(b);
                }
            }
        }
    }
    panic!("season did not reach the new-season prompt within the step budget");
}
