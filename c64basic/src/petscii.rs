//! PETSCII byte set for the C64.
//!
//! The C64 has two character sets selectable by CHR$(14) / CHR$(142):
//!   - Uppercase+graphics (default after RESET)
//!   - Lowercase+uppercase
//!
//! This module also owns the 16-colour palette and the control-code names
//! so the lexer can recognise `{clr}`, `{home}`, colour names, etc.

use crossterm::style::Color;

/// C64 colour palette, indexed 0..=15.
pub fn c64_color(i: u8) -> Color {
    // RGB approximations of the canonical VICE palette.
    match i & 0x0F {
        0 => Color::Rgb {
            r: 0x00,
            g: 0x00,
            b: 0x00,
        }, // black
        1 => Color::Rgb {
            r: 0xFF,
            g: 0xFF,
            b: 0xFF,
        }, // white
        2 => Color::Rgb {
            r: 0x88,
            g: 0x00,
            b: 0x00,
        }, // red
        3 => Color::Rgb {
            r: 0xAA,
            g: 0xFF,
            b: 0xEE,
        }, // cyan
        4 => Color::Rgb {
            r: 0xCC,
            g: 0x44,
            b: 0xCC,
        }, // purple
        5 => Color::Rgb {
            r: 0x00,
            g: 0xCC,
            b: 0x55,
        }, // green
        6 => Color::Rgb {
            r: 0x00,
            g: 0x00,
            b: 0xAA,
        }, // blue
        7 => Color::Rgb {
            r: 0xEE,
            g: 0xEE,
            b: 0x77,
        }, // yellow
        8 => Color::Rgb {
            r: 0xDD,
            g: 0x88,
            b: 0x55,
        }, // orange
        9 => Color::Rgb {
            r: 0x66,
            g: 0x44,
            b: 0x00,
        }, // brown
        10 => Color::Rgb {
            r: 0xFF,
            g: 0x77,
            b: 0x77,
        }, // light red
        11 => Color::Rgb {
            r: 0x33,
            g: 0x33,
            b: 0x33,
        }, // dark grey
        12 => Color::Rgb {
            r: 0x77,
            g: 0x77,
            b: 0x77,
        }, // medium grey
        13 => Color::Rgb {
            r: 0xAA,
            g: 0xFF,
            b: 0x66,
        }, // light green
        14 => Color::Rgb {
            r: 0x00,
            g: 0x88,
            b: 0xFF,
        }, // light blue
        15 => Color::Rgb {
            r: 0xBB,
            g: 0xBB,
            b: 0xBB,
        }, // light grey
        _ => unreachable!(),
    }
}

/// Map a PETSCII colour control byte to the palette index it selects,
/// or None if it is not a colour code.
pub fn color_code_index(b: u8) -> Option<u8> {
    match b {
        0x90 => Some(0),  // black
        0x05 => Some(1),  // white
        0x1C => Some(2),  // red
        0x9F => Some(3),  // cyan
        0x9C => Some(4),  // purple
        0x1E => Some(5),  // green
        0x1F => Some(6),  // blue
        0x9E => Some(7),  // yellow
        0x81 => Some(8),  // orange
        0x95 => Some(9),  // brown
        0x96 => Some(10), // light red
        0x97 => Some(11), // dark grey
        0x98 => Some(12), // medium grey
        0x99 => Some(13), // light green
        0x9A => Some(14), // light blue
        0x9B => Some(15), // light grey
        _ => None,
    }
}

/// Named control codes supported in petcat-style escapes inside string literals.
pub fn name_to_byte(name: &str) -> Option<u8> {
    let lower = name.to_ascii_lowercase();
    Some(match lower.as_str() {
        "wht" | "white" => 0x05,
        "dish" => 0x08,
        "ensh" => 0x09,
        "return" | "cr" => 0x0D,
        "swlc" | "lower case" | "lowercase" => 0x0E,
        "down" => 0x11,
        "rvon" | "rvs on" => 0x12,
        "home" => 0x13,
        "del" => 0x14,
        "red" => 0x1C,
        "rght" | "right" => 0x1D,
        "grn" | "green" => 0x1E,
        "blu" | "blue" => 0x1F,
        "orng" | "orange" => 0x81,
        "shift-return" | "sret" => 0x8D,
        "swuc" | "upper case" | "uppercase" => 0x8E,
        "blk" | "black" => 0x90,
        "up" => 0x91,
        "rvof" | "rvs off" => 0x92,
        "clr" | "clear" => 0x93,
        "inst" | "insert" => 0x94,
        "brn" | "brown" => 0x95,
        "lred" | "light red" => 0x96,
        "gry1" | "dkgry" | "dark grey" | "dark gray" => 0x97,
        "gry2" | "medgry" | "medium grey" | "medium gray" => 0x98,
        "lgrn" | "light green" => 0x99,
        "lblu" | "light blue" => 0x9A,
        "gry3" | "lgry" | "light grey" | "light gray" => 0x9B,
        "pur" | "purple" => 0x9C,
        "left" => 0x9D,
        "yel" | "yellow" => 0x9E,
        "cyn" | "cyan" => 0x9F,
        "space" => 0x20,
        _ => return None,
    })
}

/// Character-set mode.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Charset {
    UpperGraphics,
    LowerUpper,
}

/// Unicode glyph to display for a PETSCII screen code (not PETSCII print code).
/// Screen codes differ from print codes: PETSCII $41 ('A') prints as screen code $01.
/// We store the actual bytes placed on-screen using PETSCII print codes, and the
/// renderer translates them to Unicode at draw time.
pub fn glyph(byte: u8, cs: Charset) -> char {
    // In lowercase/uppercase mode: letters swap case, and many graphics bytes
    // become lowercase letters instead.
    if cs == Charset::LowerUpper {
        // Letters: $41..=$5A normally uppercase; in LowerUpper mode they are lowercase.
        if (0x41..=0x5A).contains(&byte) {
            return (byte + 0x20) as char;
        }
        // $C1..=$DA become uppercase letters in LowerUpper mode.
        if (0xC1..=0xDA).contains(&byte) {
            return (byte - 0x80) as char;
        }
    }

    match byte {
        // ASCII printable in both modes.
        0x20..=0x3F => byte as char,
        0x40 => '@',
        0x41..=0x5A => byte as char, // uppercase letters (UpperGraphics mode)
        0x5B => '[',
        0x5C => '£',
        0x5D => ']',
        0x5E => '↑',
        0x5F => '←',

        // Shifted / graphics block $60..=$7F: exact duplicates of $C0..=$DF
        // (both ranges map to screen codes $40..=$5F in the character ROM).
        0x60..=0x7F => glyph(byte + 0x60, cs),

        // $A0..=$BF – CBM-key graphics block (verified against chargen ROM).
        0xA0 => ' ', // shift-space
        0xA1 => '▌', // left half block
        0xA2 => '▄', // lower half block
        0xA3 => '▔', // top line
        0xA4 => '▁', // bottom line
        0xA5 => '▎', // left quarter
        0xA6 => '▒', // checkerboard
        0xA7 => '▕', // right quarter
        0xA8 => '▒', // lower-half checkerboard
        0xA9 => '◤', // upper-left triangle
        0xAA => '▕', // right quarter (dup of $A7)
        0xAB => '├',
        0xAC => '▗', // lower-right quadrant
        0xAD => '└',
        0xAE => '┐',
        0xAF => '▂', // bottom quarter
        0xB0 => '┌',
        0xB1 => '┴',
        0xB2 => '┬',
        0xB3 => '┤',
        0xB4 => '▎', // left quarter
        0xB5 => '▍', // left 3/8
        0xB6 => '▐', // right 3/8 (approx)
        0xB7 => '▔', // top quarter
        0xB8 => '▀', // top 3/8 (approx)
        0xB9 => '▄', // bottom 3/8 (approx)
        0xBA => '▟', // right edge + bottom edge
        0xBB => '▖', // lower-left quadrant
        0xBC => '▝', // upper-right quadrant
        0xBD => '┘',
        0xBE => '▘', // upper-left quadrant
        0xBF => '▚', // upper-left + lower-right quadrants

        // $C0..=$DF – shifted-letter graphics block (verified against chargen ROM).
        // Corners here are rounded on real C64 hardware.
        0xC0 => '─', // SHIFT-*
        0xC1 => '♠', // SHIFT-A
        0xC2 => '│', // SHIFT-B, centre vertical
        0xC3 => '─', // SHIFT-C, centre horizontal
        0xC4 => '─', // SHIFT-D, horizontal (one row up)
        0xC5 => '▔', // SHIFT-E, horizontal near top
        0xC6 => '─', // SHIFT-F, horizontal (one row down)
        0xC7 => '│', // SHIFT-G, vertical left of centre
        0xC8 => '│', // SHIFT-H, vertical right of centre
        0xC9 => '╮', // SHIFT-I, top-right rounded corner
        0xCA => '╰', // SHIFT-J, bottom-left rounded corner
        0xCB => '╯', // SHIFT-K, bottom-right rounded corner
        0xCC => '└', // SHIFT-L, square bottom-left (left+bottom edges)
        0xCD => '╲', // SHIFT-M, diagonal
        0xCE => '╱', // SHIFT-N, diagonal
        0xCF => '┌', // SHIFT-O, square top-left (left+top edges)
        0xD0 => '┐', // SHIFT-P, square top-right (right+top edges)
        0xD1 => '●', // SHIFT-Q, filled circle
        0xD2 => '─', // SHIFT-R, horizontal below centre
        0xD3 => '♥', // SHIFT-S
        0xD4 => '▏', // SHIFT-T, vertical near left
        0xD5 => '╭', // SHIFT-U, top-left rounded corner
        0xD6 => '╳', // SHIFT-V
        0xD7 => '○', // SHIFT-W, hollow circle
        0xD8 => '♣', // SHIFT-X
        0xD9 => '│', // SHIFT-Y, vertical right of centre
        0xDA => '♦', // SHIFT-Z
        0xDB => '┼', // SHIFT-+
        0xDC => '▌', // CBM--, left-half checkerboard (approx)
        0xDD => '│', // SHIFT--
        0xDE => 'π',
        0xDF => '◥', // upper-right triangle

        // $E0..=$FE duplicates of $A0..=$BE.
        0xE0..=0xFE => glyph(byte - 0x40, cs),
        0xFF => '~',

        _ => ' ',
    }
}
