/// PETSCII (Commodore 64 character set) to Unicode conversion
///
/// This module provides a clean array-based lookup system for converting
/// PETSCII bytes to their Unicode display equivalents or control actions.

/// PETSCII character mapping - either a printable Unicode character or a control code
#[derive(Clone, Copy)]
pub enum PetASCII {
    /// Regular printable character with Unicode equivalent
    Unicode(char),
    /// Control code (colors, cursor movement, etc.) - passed through to screen handler
    Control(u8),
}

/// Build the complete 256-entry PETSCII lookup table
pub fn build_petscii_table() -> [PetASCII; 256] {
    let mut table = [PetASCII::Unicode('?'); 256];

    // Control codes (0x00-0x1F) - pass through for screen handling
    for i in 0x00..=0x1F {
        table[i] = PetASCII::Control(i as u8);
    }

    // Standard ASCII printable (0x20-0x5F)
    for i in 0x20..=0x5F {
        table[i] = PetASCII::Unicode(i as u8 as char);
    }

    // Special handling: ASCII approximations should also map to graphics
    table[0x2D] = PetASCII::Unicode('─'); // '-' → horizontal line (from 0xC0)
    table[0x7C] = PetASCII::Unicode('│'); // '|' → vertical line (from 0xDD, 0xBC, 0xBD)
    table[0x2B] = PetASCII::Unicode('┼'); // '+' → cross (from 0xB6, etc.)
    table[0x23] = PetASCII::Unicode('█'); // '#' → full block (from 0xDB)

    // PETSCII graphics (0x60-0x7F)
    table[0x60] = PetASCII::Unicode('─'); // Horizontal line
    for i in 0x61..=0x7A {
        // Lowercase -> uppercase
        table[i] = PetASCII::Unicode((i - 0x60 + 0x41) as u8 as char);
    }
    table[0x7B] = PetASCII::Unicode('┼'); // Cross
    table[0x7C] = PetASCII::Unicode('│'); // Vertical line
    table[0x7D] = PetASCII::Unicode('┼'); // Cross variant
    table[0x7E] = PetASCII::Unicode('~'); // Tilde
    table[0x7F] = PetASCII::Unicode('·'); // Middle dot

    // Control codes (0x80-0x9F) - colors and video control
    for i in 0x80..=0x9F {
        table[i] = PetASCII::Control(i as u8);
    }

    // PETSCII box drawing (0xA0-0xBF)
    table[0xA0] = PetASCII::Unicode(' ');  // NBSP
    table[0xA1] = PetASCII::Unicode('▐');  // Right half block
    table[0xA2] = PetASCII::Unicode('░');  // Light shade
    table[0xA3] = PetASCII::Unicode('─');  // Horizontal line
    table[0xA4] = PetASCII::Unicode('▒');  // Medium shade
    table[0xA5] = PetASCII::Unicode('▔');  // Upper horizontal bar
    table[0xA6] = PetASCII::Unicode('▃');  // Lower 3/8 block
    table[0xA7] = PetASCII::Unicode('▖');  // Quadrant lower left
    table[0xA8] = PetASCII::Unicode('▝');  // Quadrant upper right
    table[0xA9] = PetASCII::Unicode('┘');  // Box bottom-right
    table[0xAA] = PetASCII::Unicode('▘');  // Quadrant upper left
    table[0xAB] = PetASCII::Unicode('╱');  // Forward slash box
    table[0xAC] = PetASCII::Unicode('▐');  // Right half block
    table[0xAD] = PetASCII::Unicode('╯');  // Box bottom-right corner
    table[0xAE] = PetASCII::Unicode('╰');  // Box bottom-left corner
    table[0xAF] = PetASCII::Unicode('╭');  // Box top-left corner
    table[0xB0] = PetASCII::Unicode('╲');  // Backslash box
    table[0xB1] = PetASCII::Unicode('├');  // Box left T-junction
    table[0xB2] = PetASCII::Unicode('┤');  // Box right T-junction
    table[0xB3] = PetASCII::Unicode('╮');  // Box top-right corner
    table[0xB4] = PetASCII::Unicode('┬');  // Box top T-junction
    table[0xB5] = PetASCII::Unicode('┴');  // Box bottom T-junction
    table[0xB6] = PetASCII::Unicode('┼');  // Box cross
    table[0xB7] = PetASCII::Unicode('◆');  // Diamond
    table[0xB8] = PetASCII::Unicode('◇');  // Hollow diamond
    table[0xB9] = PetASCII::Unicode('●');  // Filled circle (ball)
    table[0xBA] = PetASCII::Unicode('○');  // Hollow circle
    table[0xBB] = PetASCII::Unicode('┼');  // Cross variant
    table[0xBC] = PetASCII::Unicode('│');  // Vertical line
    table[0xBD] = PetASCII::Unicode('│');  // Vertical line variant
    table[0xBE] = PetASCII::Unicode('┼');  // Cross variant
    table[0xBF] = PetASCII::Unicode(' ');  // Space

    // PETSCII shifted characters (0xC0-0xDF)
    table[0xC0] = PetASCII::Unicode('─');  // Horizontal line
    for i in 0xC1..=0xDA {
        // Shifted letters A-Z
        table[i] = PetASCII::Unicode((i - 0xC0 + 0x41) as u8 as char);
    }
    table[0xDB] = PetASCII::Unicode('█');  // Full block
    table[0xDC] = PetASCII::Unicode('▄');  // Lower half block
    table[0xDD] = PetASCII::Unicode('│');  // Vertical line (CRITICAL!)
    table[0xDE] = PetASCII::Unicode('▐');  // Right half block
    table[0xDF] = PetASCII::Unicode('▄');  // Lower half block

    // PETSCII 0xE0-0xFF - various graphics
    table[0xE0] = PetASCII::Unicode('░');  // Light shade
    table[0xE1] = PetASCII::Unicode('▒');  // Medium shade
    table[0xE2] = PetASCII::Unicode('▓');  // Dark shade
    table[0xE3] = PetASCII::Unicode('◆');  // Diamond
    table[0xE4] = PetASCII::Unicode('┼');  // Cross
    table[0xE5] = PetASCII::Unicode('◄');  // Left arrow
    table[0xE6] = PetASCII::Unicode('═');  // Double horizontal
    table[0xE7] = PetASCII::Unicode('►');  // Right arrow
    table[0xE8] = PetASCII::Unicode('?');  // Question mark
    for i in 0xE9..=0xFF {
        table[i] = PetASCII::Unicode('░'); // Various graphics -> light shade
    }

    table
}

/// Convert a PETSCII byte to ASCII for use in detokenized strings
/// Returns (character, is_control_code)
/// This preserves enough information for the parser while staying single-byte
pub fn petscii_to_ascii(byte: u8) -> (char, bool) {
    // For parser compatibility, we need single-byte ASCII characters
    // Control codes are marked and skipped in detokenized output
    match byte {
        // Control codes (0x00-0x1F) - pass through as control
        0x00..=0x1F => (byte as char, true),

        // Standard ASCII range (0x20-0x5F)
        0x20..=0x5F => (byte as char, false),

        // PETSCII graphics (0x60-0x7F) - convert to approximations for parser
        0x60 => ('-', false),  // Horizontal line
        0x61..=0x7A => ((byte - 0x60 + 0x41) as char, false), // Lowercase -> uppercase
        0x7B => ('+', false),
        0x7C => ('|', false),
        0x7D => ('+', false),
        0x7E => ('~', false),
        0x7F => ('.', false),

        // Control codes (0x80-0x9F) - colors, pass through as control
        0x80..=0x9F => (byte as char, true),

        // PETSCII box drawing (0xA0-0xBF) - use ASCII approximations
        0xA0 => (' ', false),
        0xA1..=0xAF => ('+', false),
        0xB0 => ('\\', false),
        0xB1..=0xB6 => ('+', false),
        0xB7 => ('*', false),
        0xB8 => ('*', false),
        0xB9 => ('O', false), // Ball
        0xBA => ('o', false),
        0xBB..=0xBD => ('|', false),
        0xBE => ('+', false),
        0xBF => (' ', false),

        // PETSCII shifted (0xC0-0xDF) - letters and graphics
        0xC0 => ('-', false),  // Horizontal line
        0xC1..=0xDA => ((byte - 0xC0 + 0x41) as char, false), // Shifted A-Z
        0xDB => ('#', false),  // Full block
        0xDC => ('_', false),
        0xDD => ('|', false),  // Vertical line
        0xDE => (']', false),
        0xDF => ('_', false),

        // PETSCII 0xE0-0xFF
        0xE0..=0xFF => ('.', false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_petscii_table() {
        let table = build_petscii_table();

        // Test control code
        match table[0x05] {
            PetASCII::Control(5) => (),
            _ => panic!("Expected control code"),
        }

        // Test Unicode mapping
        match table[0xC0] {
            PetASCII::Unicode('─') => (),
            _ => panic!("Expected horizontal line"),
        }

        // Test vertical line (the critical one!)
        match table[0xDD] {
            PetASCII::Unicode('│') => (),
            _ => panic!("Expected vertical line"),
        }
    }
}
