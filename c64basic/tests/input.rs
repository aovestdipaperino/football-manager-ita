//! Focused tests for INPUT/GET keyboard delivery, including keys that
//! arrive before the INPUT statement arms (the C64 keyboard buffer case).

use c64basic::interp::{InputMode, Interp};
use c64basic::lang;
use c64basic::screen::Screen;

fn make(src: &str) -> Interp {
    let prog = lang::load_program(src).expect("parse");
    Interp::new(prog, Screen::new()).expect("interp")
}

fn screen_text(i: &Interp) -> String {
    i.screen.to_text()
}

#[test]
fn input_terminates_on_enter_after_arming() {
    let mut i = make("10 INPUT A$\n20 PRINT A$\n30 END\n");
    i.run_slice(100).unwrap();
    assert!(
        matches!(i.input_mode, InputMode::AwaitingLine { .. }),
        "INPUT should be waiting"
    );
    for &b in b"HELLO" {
        i.push_char(b);
    }
    i.push_char(0x0D);
    assert!(matches!(i.input_mode, InputMode::Normal));
    let halted = i.run_slice(100).unwrap();
    assert!(halted);
    assert!(screen_text(&i).contains("HELLO"));
}

#[test]
fn keys_sent_before_input_arms_are_consumed() {
    // Keys arrive while the program is still busy (delay loop), i.e. before
    // the INPUT statement runs. They must feed INPUT like the C64 keyboard
    // buffer, not strand it waiting forever.
    let mut i = make("10 FORT=1TO100:NEXT\n20 INPUT A$\n30 PRINT A$\n40 END\n");
    for &b in b"15\r" {
        i.push_char(b);
    }
    let halted = i.run_slice(10_000).unwrap();
    assert!(halted, "program should run to completion");
    assert!(screen_text(&i).contains("15"));
}

#[test]
fn queued_bytes_after_enter_are_kept_for_later_statements() {
    // "8\rG" queued early: INPUT takes "8", the trailing G must remain
    // available for the GET that follows.
    let mut i = make("10 INPUT A$\n20 GET B$:IFB$=\"\"THEN20\n30 PRINT A$;B$\n40 END\n");
    for &b in b"8\rG" {
        i.push_char(b);
    }
    let halted = i.run_slice(10_000).unwrap();
    assert!(halted);
    assert!(screen_text(&i).contains("8G"));
}

#[test]
fn backspace_edits_the_input_buffer() {
    let mut i = make("10 INPUT A$\n20 PRINT \">\";A$;\"<\"\n30 END\n");
    i.run_slice(100).unwrap();
    for &b in b"AB" {
        i.push_char(b);
    }
    i.push_char(0x14); // DEL
    i.push_char(b'C');
    i.push_char(0x0D);
    i.run_slice(100).unwrap();
    assert!(screen_text(&i).contains(">AC<"));
}

#[test]
fn split_delivery_across_slices() {
    let mut i = make("10 INPUT A$\n20 PRINT A$\n30 END\n");
    i.run_slice(100).unwrap();
    i.push_char(b'4');
    i.run_slice(100).unwrap(); // still waiting
    assert!(matches!(i.input_mode, InputMode::AwaitingLine { .. }));
    i.push_char(b'2');
    i.push_char(0x0D);
    let halted = i.run_slice(100).unwrap();
    assert!(halted);
    assert!(screen_text(&i).contains("42"));
}

#[test]
fn get_consumes_queued_keys_in_order() {
    let mut i =
        make("10 GET A$:IFA$=\"\"THEN10\n20 GET B$:IFB$=\"\"THEN20\n30 PRINT A$;B$\n40 END\n");
    i.push_char(b'X');
    i.push_char(b'Y');
    let halted = i.run_slice(10_000).unwrap();
    assert!(halted);
    assert!(screen_text(&i).contains("XY"));
}
