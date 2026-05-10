/// Convert a Commodore 64 .PRG BASIC file to a petcat-compatible plain-text listing.
///
/// Output format follows VICE's petcat conventions:
/// - Control codes are emitted as {name} (e.g. {clr}, {home}, {down}, {rvon})
/// - Colour codes use petcat colour names (e.g. {wht}, {blk}, {cyn})
/// - Graphic / reverse bytes without a dedicated name are emitted as {$XX}
///
/// The result can be re-tokenised with: `petcat -w2 -o game.prg -- game.txt`
///
/// Usage: prg_to_petcat <program.prg>  (writes to stdout)
use basic_emulator::prg_loader::load_prg_file;
use std::env;

const TOKENS: &[&str] = &[
    "END", "FOR", "NEXT", "DATA", "INPUT#", "INPUT", "DIM", "READ", "LET", "GOTO", "RUN", "IF",
    "RESTORE", "GOSUB", "RETURN", "REM", "STOP", "ON", "WAIT", "LOAD", "SAVE", "VERIFY", "DEF",
    "POKE", "PRINT#", "PRINT", "CONT", "LIST", "CLR", "CMD", "SYS", "OPEN", "CLOSE", "GET", "NEW",
    "TAB(", "TO", "FN", "SPC(", "THEN", "NOT", "STEP", "+", "-", "*", "/", "^", "AND", "OR", ">",
    "=", "<", "SGN", "INT", "ABS", "USR", "FRE", "POS", "SQR", "RND", "LOG", "EXP", "COS", "SIN",
    "TAN", "ATN", "PEEK", "LEN", "STR$", "VAL", "ASC", "CHR$", "LEFT$", "RIGHT$", "MID$", "GO",
];

/// Return petcat-style tag for a PETSCII control byte, or None for printable bytes.
fn petcat_name(b: u8) -> Option<&'static str> {
    match b {
        0x05 => Some("wht"),
        0x08 => Some("dish"),
        0x09 => Some("ensh"),
        0x0D => Some("return"),
        0x0E => Some("swlc"),
        0x11 => Some("down"),
        0x12 => Some("rvon"),
        0x13 => Some("home"),
        0x14 => Some("del"),
        0x1C => Some("red"),
        0x1D => Some("rght"),
        0x1E => Some("grn"),
        0x1F => Some("blu"),
        0x81 => Some("orng"),
        0x8D => Some("shift-return"),
        0x8E => Some("swuc"),
        0x90 => Some("blk"),
        0x91 => Some("up"),
        0x92 => Some("rvof"),
        0x93 => Some("clr"),
        0x94 => Some("inst"),
        0x95 => Some("brn"),
        0x96 => Some("lred"),
        0x97 => Some("gry1"),
        0x98 => Some("gry2"),
        0x99 => Some("lgrn"),
        0x9A => Some("lblu"),
        0x9B => Some("gry3"),
        0x9C => Some("pur"),
        0x9D => Some("left"),
        0x9E => Some("yel"),
        0x9F => Some("cyn"),
        _ => None,
    }
}

fn emit_literal_byte(out: &mut String, b: u8) {
    // Inside strings / REM / DATA we preserve every byte losslessly.
    match b {
        // Printable 7-bit ASCII range that survives petcat round-trip untouched.
        0x20..=0x21 | 0x23..=0x7E => out.push(b as char),
        // Double-quote cannot appear inside a PETSCII string literal as a raw byte;
        // represent it as {$22} so the re-tokeniser preserves it without ending the string.
        0x22 => out.push_str("{$22}"),
        _ => {
            if let Some(name) = petcat_name(b) {
                out.push('{');
                out.push_str(name);
                out.push('}');
            } else {
                out.push_str(&format!("{{${:02x}}}", b));
            }
        }
    }
}

fn detokenize(bytes: &[u8]) -> Result<String, String> {
    let mut out = String::new();
    let mut pos = 0usize;

    loop {
        if pos + 1 >= bytes.len() {
            break;
        }
        let link_lo = bytes[pos];
        let link_hi = bytes[pos + 1];
        pos += 2;
        if link_lo == 0 && link_hi == 0 {
            break;
        }

        if pos + 1 >= bytes.len() {
            return Err("truncated line number".into());
        }
        let line = bytes[pos] as u16 | ((bytes[pos + 1] as u16) << 8);
        pos += 2;

        out.push_str(&format!("{} ", line));

        let mut in_quotes = false;
        let mut in_rem = false;
        let mut in_data = false;

        while pos < bytes.len() && bytes[pos] != 0 {
            let b = bytes[pos];
            pos += 1;

            if b == b'"' {
                in_quotes = !in_quotes;
                out.push('"');
                continue;
            }

            if in_quotes {
                emit_literal_byte(&mut out, b);
                continue;
            }

            if in_rem {
                emit_literal_byte(&mut out, b);
                continue;
            }

            if b >= 0x80 {
                let idx = (b - 0x80) as usize;
                if idx < TOKENS.len() {
                    let kw = TOKENS[idx];
                    out.push_str(kw);
                    if kw == "REM" {
                        in_rem = true;
                    } else if kw == "DATA" {
                        in_data = true;
                    }
                } else {
                    // Unknown token – preserve as raw hex.
                    out.push_str(&format!("{{${:02x}}}", b));
                }
                continue;
            }

            // Outside strings/REM, bytes < $80 are either ASCII or embedded PETSCII
            // control/graphic bytes. DATA lines after the keyword are free-form text
            // that must also preserve exotic bytes.
            if in_data {
                emit_literal_byte(&mut out, b);
                continue;
            }

            match b {
                0x20..=0x7E => out.push(b as char),
                _ => emit_literal_byte(&mut out, b),
            }
        }

        // Skip line terminator.
        if pos < bytes.len() && bytes[pos] == 0 {
            pos += 1;
        }
        out.push('\n');
    }

    Ok(out)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <program.prg>", args[0]);
        std::process::exit(1);
    }

    let bytes = load_prg_file(&args[1]).unwrap_or_else(|e| {
        eprintln!("cannot read {}: {}", args[1], e);
        std::process::exit(1);
    });

    match detokenize(&bytes) {
        Ok(s) => print!("{}", s),
        Err(e) => {
            eprintln!("detokenise error: {}", e);
            std::process::exit(1);
        }
    }
}
