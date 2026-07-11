//! Seeding the interpreter makes RND reproducible, so a scripted run of the
//! game produces the same screen every time. This is what lets golden tests
//! reach past the deterministic menus into simulated matches.

use c64basic::interp::{InputMode, Interp};
use c64basic::lang;
use c64basic::screen::Screen;

fn load_seeded(seed: u64) -> Interp {
    let src = std::fs::read_to_string(format!(
        "{}/../footballmanager.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("footballmanager.txt");
    let prog = lang::load_program(&src).expect("parse");
    Interp::new_seeded(prog, Screen::new(), seed).expect("interp")
}

/// Drive the game to the team menu, pick team 8, play one match to the first
/// half-time, and return the resulting screen text.
fn play_to_halftime(seed: u64) -> String {
    let mut i = load_seeded(seed);
    let feed = |i: &mut Interp, s: &[u8]| {
        for &b in s {
            i.push_char(b);
        }
    };
    // team selection INPUT
    for _ in 0..200 {
        i.run_slice(200_000).unwrap();
        if !matches!(i.input_mode, InputMode::Normal) {
            break;
        }
    }
    feed(&mut i, b"8\r");
    // main menu GET -> G, then the pre-match SPACE prompts (GET loops)
    for keys in [&b"G"[..], b" ", b" ", b" "] {
        i.run_slice(400_000).unwrap();
        feed(&mut i, keys);
    }
    i.run_slice(1_000_000).unwrap();
    i.screen.to_text()
}

#[test]
fn same_seed_gives_same_run() {
    let a = play_to_halftime(12345);
    let b = play_to_halftime(12345);
    assert_eq!(a, b, "identical seeds must produce identical screens");
}

#[test]
fn rnd_is_reproducible_in_isolation() {
    fn rolls(seed: u64) -> Vec<i64> {
        let prog = lang::load_program("10 FORI=1TO8:PRINTINT(RND(1)*100);:NEXT\n20 END\n").unwrap();
        let mut i = Interp::new_seeded(prog, Screen::new(), seed).unwrap();
        i.run_slice(10_000).unwrap();
        i.screen
            .to_text()
            .split_whitespace()
            .filter_map(|t| t.parse::<i64>().ok())
            .collect()
    }
    assert_eq!(rolls(7), rolls(7));
    assert_ne!(rolls(7), rolls(8), "different seeds should diverge");
}
