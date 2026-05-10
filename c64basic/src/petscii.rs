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

        // Shifted / graphics block $60..=$7F (duplicates of $C0..=$DF).
        0x60 => ' ', // NBSP-like
        0x61 => '▌',
        0x62 => '▄',
        0x63 => '▔',
        0x64 => '▁',
        0x65 => '▏',
        0x66 => '▒',
        0x67 => '▕',
        0x68 => '▓',
        0x69 => '◤',
        0x6A => '▗',
        0x6B => '├',
        0x6C => '▝',
        0x6D => '╰',
        0x6E => '╮',
        0x6F => '▂',
        0x70 => '╭',
        0x71 => '┴',
        0x72 => '┬',
        0x73 => '┤',
        0x74 => '▎',
        0x75 => '▍',
        0x76 => '▉',
        0x77 => '▊',
        0x78 => '▋',
        0x79 => '▀',
        0x7A => '▐',
        0x7B => '▃',
        0x7C => '✓',
        0x7D => '╯',
        0x7E => '▘',
        0x7F => '▚',

        // $A0..=$BF graphics.
        0xA0 => ' ',
        0xA1 => '▏',
        0xA2 => '▕',
        0xA3 => '▔',
        0xA4 => '▁',
        0xA5 => '▎',
        0xA6 => '▒',
        0xA7 => '▕',
        0xA8 => '◤',
        0xA9 => '◥',
        0xAA => '├',
        0xAB => '└',
        0xAC => '┐',
        0xAD => '╰',
        0xAE => '╮',
        0xAF => '▂',
        0xB0 => '╭',
        0xB1 => '┴',
        0xB2 => '┬',
        0xB3 => '┤',
        0xB4 => '▎',
        0xB5 => '▍',
        0xB6 => '▋',
        0xB7 => '▊',
        0xB8 => '▉',
        0xB9 => '▂',
        0xBA => '▎',
        0xBB => '▃',
        0xBC => '│',
        0xBD => '╯',
        0xBE => '▖',
        0xBF => '◥',

        // $C0..=$DF – canonical graphics block (shifted letter PETSCII glyphs).
        // Corners here are rounded on real C64 hardware.
        0xC0 => '─',
        0xC1 => '♠',
        0xC2 => '│', // left half of vert bar
        0xC3 => '─',
        0xC4 => '─', // upper half horizontal
        0xC5 => '─',
        0xC6 => '│',
        0xC7 => '│',
        0xC8 => '│',
        0xC9 => '╮', // top-right rounded corner
        0xCA => '╰', // bottom-left rounded corner
        0xCB => '╯', // bottom-right rounded corner
        0xCC => '├',
        0xCD => '╯', // variant bottom-right
        0xCE => '╮', // variant top-right
        0xCF => '●',
        0xD0 => '♣',
        0xD1 => '│',
        0xD2 => '♥',
        0xD3 => '╭', // variant top-left
        0xD4 => '╳',
        0xD5 => '╭', // top-left rounded corner
        0xD6 => '╳',
        0xD7 => '○',
        0xD8 => '♦',
        0xD9 => 'π',
        0xDA => '╮', // variant top-right
        0xDB => '█',
        0xDC => '▌',
        0xDD => '│',
        0xDE => '▒',
        0xDF => '▓',

        // $E0..=$FE duplicates of $A0..=$BE.
        0xE0..=0xFE => glyph(byte - 0x40, cs),
        0xFF => '~',

        _ => ' ',
    }
}
