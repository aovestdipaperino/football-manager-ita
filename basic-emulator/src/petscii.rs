/// PETSCII (Commodore 64 character set) to Unicode conversion
///
/// PETSCII has special characters for:
/// - Box drawing (0xA0-0xBF range and others)
/// - Control codes (0x00-0x1F for colors, reverse video, etc.)
/// - Graphics characters (0xC0-0xDF for various blocks and symbols)

use std::collections::HashMap;

/// Convert a PETSCII byte to an ASCII-safe character or control sequence
/// Returns (character, is_control_code)
/// Uses ASCII approximations to avoid multi-byte UTF-8 issues in the parser
///
/// NOTE: Control codes are returned as-is (with is_control=true) so they can
/// be processed by the screen module for cursor movement, colors, etc.
pub fn petscii_to_ascii(byte: u8) -> (char, bool) {
    match byte {
        // Control codes (0x00-0x1F) - pass through for screen handling
        0x00..=0x1F => (byte as char, true),

        // Standard ASCII range (0x20-0x5F)
        0x20..=0x5F => (byte as char, false),

        // PETSCII graphics characters (0x60-0x7F) - use ASCII approximations
        0x60 => ('-', false), // Horizontal line
        0x61..=0x7A => ((byte - 0x60 + 0x41) as char, false), // Lowercase -> uppercase
        0x7B => ('+', false),
        0x7C => ('|', false),
        0x7D => ('+', false),
        0x7E => ('~', false),
        0x7F => ('.', false),

        // Control codes (0x80-0x9F) - pass through for screen handling
        0x80..=0x9F => (byte as char, true),

        // PETSCII box drawing and graphics (0xA0-0xBF) - ASCII approximations
        0xA0 => (' ', false), // NBSP / Shifted space
        0xAB => ('/', false), // Forward slash box
        0xAC => ('<', false),
        0xAD => ('+', false), // Box bottom-right
        0xAE => ('+', false), // Box bottom-left
        0xAF => ('+', false), // Box top-left
        0xB0 => ('\\', false), // Backslash box
        0xB1 => ('+', false), // Box left T
        0xB2 => ('+', false), // Box right T
        0xB3 => ('+', false), // Box top-right corner
        0xB4 => ('+', false), // Box top T
        0xB5 => ('+', false), // Box bottom T
        0xB6 => ('+', false), // Box cross
        0xB7 => ('*', false),
        0xB8 => ('*', false),
        0xB9 => ('*', false),
        0xBA => ('*', false),
        0xBB => ('+', false),
        0xBC => ('+', false),
        0xBD => ('|', false),
        0xBE => ('+', false),
        0xBF => (' ', false),

        // PETSCII shifted characters (0xC0-0xDF)
        // These are graphics characters and shifted alphabet
        0xC0 => ('-', false), // Horizontal line
        0xC1 => ('A', false), // Shifted A
        0xC2 => ('B', false),
        0xC3 => ('C', false),
        0xC4 => ('D', false),
        0xC5 => ('E', false),
        0xC6 => ('F', false),
        0xC7 => ('G', false),
        0xC8 => ('H', false),
        0xC9 => ('I', false),
        0xCA => ('J', false),
        0xCB => ('K', false),
        0xCC => ('L', false),
        0xCD => ('M', false),
        0xCE => ('N', false),
        0xCF => ('O', false),
        0xD0 => ('P', false),
        0xD1 => ('Q', false),
        0xD2 => ('R', false),
        0xD3 => ('S', false),
        0xD4 => ('T', false),
        0xD5 => ('U', false),
        0xD6 => ('V', false),
        0xD7 => ('W', false),
        0xD8 => ('X', false),
        0xD9 => ('Y', false),
        0xDA => ('Z', false),
        0xDB => ('#', false), // Full block
        0xDC => ('_', false), // Lower half block
        0xDD => ('[', false), // Left half block
        0xDE => (']', false), // Right half block
        0xDF => ('_', false), // Lower half block

        // More shifted characters (0xE0-0xFF)
        0xE0..=0xFE => ('.', false), // Various graphics
        0xFF => ('.', false),

        // Everything else - use replacement character
        _ => ('?', false),
    }
}

/// Convert a PETSCII byte to a Unicode character or control sequence
/// Returns (character, is_control_code)
pub fn petscii_to_unicode(byte: u8) -> (char, bool) {
    match byte {
        // Control codes (0x00-0x1F)
        0x05 => ('█', false), // White color (we'll just show a block)
        0x1C => ('█', false), // Red
        0x1E => ('█', false), // Green
        0x1F => ('█', false), // Blue
        0x81 => ('█', false), // Orange
        0x90 => (' ', true),   // Black (reverse off) - control code
        0x12 => (' ', true),   // Reverse on
        0x92 => (' ', true),   // Reverse off
        0x8E => (' ', true),   // Lowercase/uppercase switch
        0x8F => (' ', true),   // Switch to uppercase

        // Standard ASCII range (0x20-0x5F)
        0x20..=0x5F => (byte as char, false),

        // PETSCII graphics characters (0x60-0x7F)
        0x60 => ('─', false), // Horizontal line
        0x61..=0x7F => ('░', false), // Various graphics - use light shade

        // PETSCII box drawing and graphics (0xA0-0xBF)
        0xA0 => (' ', false), // NBSP / Shifted space
        0xAB => ('╱', false), // Forward slash box
        0xB0 => ('╲', false), // Backslash box
        0xB3 => ('╮', false), // Box top-right corner
        0xAD => ('╯', false), // Box bottom-right
        0xAE => ('╰', false), // Box bottom-left
        0xAF => ('╭', false), // Box top-left
        0xB1 => ('├', false), // Box left T
        0xB2 => ('┤', false), // Box right T
        0xB4 => ('┬', false), // Box top T
        0xB5 => ('┴', false), // Box bottom T
        0xB6 => ('┼', false), // Box cross

        // More PETSCII graphics (0xC0-0xDF)
        0xC0 => ('─', false), // Horizontal line (thick)
        0xC1..=0xCF => ('▀', false), // Upper half block
        0xD0..=0xDA => ('▄', false), // Lower half block
        0xDB => ('█', false), // Full block (0xDB specifically)
        0xDC => ('▄', false), // Lower half block
        0xDD => ('▌', false), // Left half block
        0xDE => ('▐', false), // Right half block
        0xDF => ('▄', false), // Lower half block

        // Everything else - use replacement character
        _ => ('?', false),
    }
}

/// Build a complete PETSCII to Unicode lookup table
pub fn build_petscii_table() -> HashMap<u8, char> {
    let mut table = HashMap::new();

    // Populate with all mappings
    for byte in 0u8..=255 {
        let (ch, _) = petscii_to_unicode(byte);
        table.insert(byte, ch);
    }

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ascii() {
        assert_eq!(petscii_to_unicode(0x41).0, 'A');
        assert_eq!(petscii_to_unicode(0x20).0, ' ');
    }

    #[test]
    fn test_graphics() {
        assert_eq!(petscii_to_unicode(0xC0).0, '─');
        assert_eq!(petscii_to_unicode(0xDB).0, '█');
    }

    #[test]
    fn test_control_codes() {
        let (_, is_ctrl) = petscii_to_unicode(0x90);
        assert!(is_ctrl);
    }
}
