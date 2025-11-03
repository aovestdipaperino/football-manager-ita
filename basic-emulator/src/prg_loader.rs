/// PRG file loader and detokenizer for Commodore 64 BASIC programs
///
/// PRG file format:
/// - 2-byte load address (little-endian, typically $0801)
/// - Tokenized BASIC program:
///   - Link pointer (2 bytes, little-endian) - points to next line or $0000 for end
///   - Line number (2 bytes, big-endian)
///   - Tokenized content (tokens $80-$FF, ASCII text)
///   - Line terminator ($00)

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Commodore 64 BASIC V2 token table
/// Tokens start at 128 ($80) and map to reserved words
const TOKENS: &[&str] = &[
    "END",      // 128 / $80
    "FOR",      // 129 / $81
    "NEXT",     // 130 / $82
    "DATA",     // 131 / $83
    "INPUT#",   // 132 / $84
    "INPUT",    // 133 / $85
    "DIM",      // 134 / $86
    "READ",     // 135 / $87
    "LET",      // 136 / $88
    "GOTO",     // 137 / $89
    "RUN",      // 138 / $8A
    "IF",       // 139 / $8B
    "RESTORE",  // 140 / $8C
    "GOSUB",    // 141 / $8D
    "RETURN",   // 142 / $8E
    "REM",      // 143 / $8F
    "STOP",     // 144 / $90
    "ON",       // 145 / $91
    "WAIT",     // 146 / $92
    "LOAD",     // 147 / $93
    "SAVE",     // 148 / $94
    "VERIFY",   // 149 / $95
    "DEF",      // 150 / $96
    "POKE",     // 151 / $97
    "PRINT#",   // 152 / $98
    "PRINT",    // 153 / $99
    "CONT",     // 154 / $9A
    "LIST",     // 155 / $9B
    "CLR",      // 156 / $9C
    "CMD",      // 157 / $9D
    "SYS",      // 158 / $9E
    "OPEN",     // 159 / $9F
    "CLOSE",    // 160 / $A0
    "GET",      // 161 / $A1
    "NEW",      // 162 / $A2
    "TAB(",     // 163 / $A3
    "TO",       // 164 / $A4
    "FN",       // 165 / $A5
    "SPC(",     // 166 / $A6
    "THEN",     // 167 / $A7
    "NOT",      // 168 / $A8
    "STEP",     // 169 / $A9
    "+",        // 170 / $AA
    "-",        // 171 / $AB
    "*",        // 172 / $AC
    "/",        // 173 / $AD
    "^",        // 174 / $AE
    "AND",      // 175 / $AF
    "OR",       // 176 / $B0
    ">",        // 177 / $B1
    "=",        // 178 / $B2
    "<",        // 179 / $B3
    "SGN",      // 180 / $B4
    "INT",      // 181 / $B5
    "ABS",      // 182 / $B6
    "USR",      // 183 / $B7
    "FRE",      // 184 / $B8
    "POS",      // 185 / $B9
    "SQR",      // 186 / $BA
    "RND",      // 187 / $BB
    "LOG",      // 188 / $BC
    "EXP",      // 189 / $BD
    "COS",      // 190 / $BE
    "SIN",      // 191 / $BF
    "TAN",      // 192 / $C0
    "ATN",      // 193 / $C1
    "PEEK",     // 194 / $C2
    "LEN",      // 195 / $C3
    "STR$",     // 196 / $C4
    "VAL",      // 197 / $C5
    "ASC",      // 198 / $C6
    "CHR$",     // 199 / $C7
    "LEFT$",    // 200 / $C8
    "RIGHT$",   // 201 / $C9
    "MID$",     // 202 / $CA
    "GO",       // 203 / $CB
];

/// Load a PRG file and return the raw bytes after the load address
pub fn load_prg_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    if bytes.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PRG file too short (missing load address)",
        ));
    }

    // Skip the 2-byte load address at the beginning
    // Load address is typically $0801 (2049) for BASIC programs
    let load_address = u16::from_le_bytes([bytes[0], bytes[1]]);
    eprintln!("PRG load address: ${:04X}", load_address);

    Ok(bytes[2..].to_vec())
}

/// Detokenize a Commodore 64 BASIC program to ASCII text
pub fn detokenize_program(bytes: &[u8]) -> Result<String, String> {
    let mut result = String::new();
    let mut pos = 0;

    while pos < bytes.len() {
        // Read link pointer (2 bytes, little-endian)
        if pos + 1 >= bytes.len() {
            break;
        }

        let link_lo = bytes[pos];
        let link_hi = bytes[pos + 1];
        pos += 2;

        // Check for end of program (link pointer = $0000)
        if link_lo == 0 && link_hi == 0 {
            break;
        }

        // Read line number (2 bytes, little-endian in memory, but logically big-endian)
        // C64 stores line numbers in little-endian format in memory
        if pos + 1 >= bytes.len() {
            return Err("Unexpected end of file while reading line number".to_string());
        }

        let line_lo = bytes[pos] as u16;
        let line_hi = bytes[pos + 1] as u16;
        pos += 2;

        let line_number = line_lo | (line_hi << 8);

        // Write line number
        result.push_str(&format!("{} ", line_number));

        // Process tokens and text until null terminator
        let mut in_quotes = false;
        let mut in_rem = false;
        let mut last_was_alphanumeric = false;

        while pos < bytes.len() && bytes[pos] != 0 {
            let byte = bytes[pos];
            pos += 1;

            // Check for quote character
            if byte == b'"' {
                in_quotes = !in_quotes;
                result.push('"');
                last_was_alphanumeric = false;
                continue;
            }

            // Inside quotes or REM, treat everything as literal
            if in_quotes || in_rem {
                // Convert PETSCII to ASCII
                let ch = petscii_to_ascii(byte);
                result.push(ch);
                last_was_alphanumeric = false;
                continue;
            }

            if byte >= 128 {
                // This is a token
                let token_index = (byte - 128) as usize;
                if token_index < TOKENS.len() {
                    let token = TOKENS[token_index];

                    // Add space BEFORE certain keywords if preceded by alphanumeric
                    let needs_space_before = matches!(
                        token,
                        "IF" | "FOR" | "THEN" | "TO" | "STEP" | "AND" | "OR" | "NOT" |
                        "GOTO" | "GOSUB" | "ON" | "DIM" | "READ" | "INPUT" | "LET" |
                        "PRINT" | "GET" | "NEXT" | "DATA" | "RETURN" | "END"
                    );

                    if last_was_alphanumeric && needs_space_before {
                        result.push(' ');
                    }

                    result.push_str(token);

                    // Check if this is a REM statement (disables tokenization for rest of line)
                    if token == "REM" {
                        in_rem = true;
                    }

                    // Add a space after certain keywords if the next byte is a letter or digit
                    // This makes the output more compatible with parsers that expect spaces
                    if pos < bytes.len() {
                        let next_byte = bytes[pos];
                        let next_is_alphanumeric = (b'A'..=b'Z').contains(&next_byte) ||
                                                   (b'a'..=b'z').contains(&next_byte) ||
                                                   (b'0'..=b'9').contains(&next_byte);

                        // Always add space after these tokens when followed by alphanumeric
                        let needs_space_after = matches!(
                            token,
                            "IF" | "FOR" | "THEN" | "TO" | "STEP" | "AND" | "OR" | "NOT" |
                            "GOTO" | "GOSUB" | "ON" | "DIM" | "READ" | "INPUT" | "LET" |
                            "PRINT" | "GET" | "NEXT" | "DATA" | "RETURN" | "END"
                        );

                        if next_is_alphanumeric && needs_space_after {
                            result.push(' ');
                            last_was_alphanumeric = false;
                        } else {
                            last_was_alphanumeric = false;
                        }
                    } else {
                        last_was_alphanumeric = false;
                    }
                } else {
                    return Err(format!(
                        "Unknown token ${:02X} at position {} (line {})",
                        byte, pos - 1, line_number
                    ));
                }
            } else {
                // Regular ASCII character
                let ch = petscii_to_ascii(byte);
                result.push(ch);

                // Track if this character is alphanumeric
                last_was_alphanumeric = (b'A'..=b'Z').contains(&byte) ||
                                       (b'a'..=b'z').contains(&byte) ||
                                       (b'0'..=b'9').contains(&byte);
            }
        }

        // Skip the null terminator
        if pos < bytes.len() && bytes[pos] == 0 {
            pos += 1;
        }

        // Add newline
        result.push('\n');
    }

    Ok(result)
}

/// Convert PETSCII character to ASCII
/// This is a simplified conversion for printable characters
fn petscii_to_ascii(petscii: u8) -> char {
    match petscii {
        // Printable ASCII range is mostly compatible
        32..=95 => petscii as char,
        // Lowercase letters in PETSCII
        97..=122 => petscii as char,
        // Map some common PETSCII codes
        _ => '?', // Unknown characters become '?'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detokenize_simple() {
        // 10 PRINT "HELLO"
        // Format: [link_lo] [link_hi] [line_lo] [line_hi] [PRINT token] [space] ["] [H] [E] [L] [L] [O] ["] [null]
        let bytes = vec![
            0x10, 0x08,  // Link pointer (points to next line)
            0x0A, 0x00,  // Line number 10 (little-endian)
            153,         // PRINT token ($99)
            32,          // Space
            34,          // "
            b'H', b'E', b'L', b'L', b'O',
            34,          // "
            0,           // Line terminator
            0x00, 0x00,  // End of program marker
        ];

        let result = detokenize_program(&bytes).unwrap();
        assert!(result.contains("10 PRINT"));
        assert!(result.contains("HELLO"));
    }

    #[test]
    fn test_detokenize_with_tokens() {
        // 20 FOR I=1 TO 10
        let bytes = vec![
            0x20, 0x08,  // Link pointer
            0x14, 0x00,  // Line number 20 (little-endian)
            129,         // FOR token ($81)
            32,          // Space
            b'I',
            178,         // = token ($B2)
            b'1',
            164,         // TO token ($A4)
            b'1', b'0',
            0,           // Line terminator
            0x00, 0x00,  // End of program marker
        ];

        let result = detokenize_program(&bytes).unwrap();
        assert!(result.contains("20 FOR"));
        assert!(result.contains("I=1TO10"));
    }
}
