/// PETSCII (Commodore 64 character set) to Unicode conversion
///
/// This module provides a clean array-based lookup system for converting
/// PETSCII bytes to their Unicode display equivalents or control actions.
///
/// The C64 has two character sets that can be switched:
/// - Charset 0 (uppercase/graphics): Letters display as UPPERCASE, codes 0x60-0xFF are graphics
/// - Charset 1 (lowercase/uppercase): Letters can be lowercase, codes 0x60-0xFF are letters
///
/// Control codes to switch:
/// - CHR$(14) / 0x0E: Switch to uppercase/graphics (charset 0)
/// - CHR$(142) / 0x8E: Switch to lowercase/uppercase (charset 1)

/// PETSCII character mapping - either a printable Unicode character or a control code
#[derive(Clone, Copy)]
pub enum PetASCII {
    /// Regular printable character with Unicode equivalent
    Unicode(char),
    /// Control code (colors, cursor movement, etc.) - passed through to screen handler
    Control(u8),
}

/// Character set mode
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CharsetMode {
    /// Uppercase/Graphics mode (default) - graphics in 0x60-0xFF range
    UppercaseGraphics,
    /// Lowercase/Uppercase mode - lowercase letters available
    LowercaseUppercase,
}

/// Build the complete 256-entry PETSCII lookup table for uppercase/graphics mode
pub fn build_petscii_table_uppercase() -> [PetASCII; 256] {
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

    // PETSCII graphics (0x60-0x7F) - UPPERCASE/GRAPHICS mode
    // These are DUPLICATES of 0xC0-0xDF
    table[0x60] = PetASCII::Unicode('─'); // horizontal line (top half missing)
    table[0x61] = PetASCII::Unicode('│'); // vertical line (left half missing)
    table[0x62] = PetASCII::Unicode('─'); // horizontal line (bottom half missing)
    table[0x63] = PetASCII::Unicode('│'); // vertical line (right half missing)
    table[0x64] = PetASCII::Unicode('╱'); // diagonal slash
    table[0x65] = PetASCII::Unicode('╲'); // diagonal backslash
    table[0x66] = PetASCII::Unicode('◢'); // triangle lower right
    table[0x67] = PetASCII::Unicode('◣'); // triangle lower left
    table[0x68] = PetASCII::Unicode('░'); // light checkerboard
    table[0x69] = PetASCII::Unicode('▒'); // medium checkerboard
    table[0x6A] = PetASCII::Unicode('▌'); // left half block
    table[0x6B] = PetASCII::Unicode('┌'); // top-left corner
    table[0x6C] = PetASCII::Unicode('┐'); // top-right corner
    table[0x6D] = PetASCII::Unicode('└'); // bottom-left corner
    table[0x6E] = PetASCII::Unicode('┘'); // bottom-right corner
    table[0x6F] = PetASCII::Unicode('├'); // left T-junction
    table[0x70] = PetASCII::Unicode('┤'); // right T-junction
    table[0x71] = PetASCII::Unicode('┬'); // top T-junction
    table[0x72] = PetASCII::Unicode('┴'); // bottom T-junction
    table[0x73] = PetASCII::Unicode('┼'); // cross
    table[0x74] = PetASCII::Unicode('●'); // filled circle
    table[0x75] = PetASCII::Unicode('○'); // hollow circle
    table[0x76] = PetASCII::Unicode('▐'); // right half block
    table[0x77] = PetASCII::Unicode('▀'); // upper half block
    table[0x78] = PetASCII::Unicode('▄'); // lower half block
    table[0x79] = PetASCII::Unicode('█'); // full block
    table[0x7A] = PetASCII::Unicode('◤'); // triangle upper left
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
    table[0xA0] = PetASCII::Unicode(' '); // NBSP
    table[0xA1] = PetASCII::Unicode('▐'); // Right half block
    table[0xA2] = PetASCII::Unicode('░'); // Light shade
    table[0xA3] = PetASCII::Unicode('─'); // Horizontal line
    table[0xA4] = PetASCII::Unicode('▒'); // Medium shade
    table[0xA5] = PetASCII::Unicode('▔'); // Upper horizontal bar
    table[0xA6] = PetASCII::Unicode('▃'); // Lower 3/8 block
    table[0xA7] = PetASCII::Unicode('▖'); // Quadrant lower left
    table[0xA8] = PetASCII::Unicode('▝'); // Quadrant upper right
    table[0xA9] = PetASCII::Unicode('┘'); // Box bottom-right
    table[0xAA] = PetASCII::Unicode('▘'); // Quadrant upper left
    table[0xAB] = PetASCII::Unicode('╱'); // Forward slash box
    table[0xAC] = PetASCII::Unicode('▐'); // Right half block
    table[0xAD] = PetASCII::Unicode('╯'); // Box bottom-right corner
    table[0xAE] = PetASCII::Unicode('╰'); // Box bottom-left corner
    table[0xAF] = PetASCII::Unicode('╭'); // Box top-left corner
    table[0xB0] = PetASCII::Unicode('╲'); // Backslash box
    table[0xB1] = PetASCII::Unicode('├'); // Box left T-junction
    table[0xB2] = PetASCII::Unicode('┤'); // Box right T-junction
    table[0xB3] = PetASCII::Unicode('╮'); // Box top-right corner
    table[0xB4] = PetASCII::Unicode('┬'); // Box top T-junction
    table[0xB5] = PetASCII::Unicode('┴'); // Box bottom T-junction
    table[0xB6] = PetASCII::Unicode('┼'); // Box cross
    table[0xB7] = PetASCII::Unicode('◆'); // Diamond
    table[0xB8] = PetASCII::Unicode('◇'); // Hollow diamond
    table[0xB9] = PetASCII::Unicode('●'); // Filled circle (ball)
    table[0xBA] = PetASCII::Unicode('○'); // Hollow circle
    table[0xBB] = PetASCII::Unicode('┼'); // Cross variant
    table[0xBC] = PetASCII::Unicode('│'); // Vertical line
    table[0xBD] = PetASCII::Unicode('│'); // Vertical line variant
    table[0xBE] = PetASCII::Unicode('┼'); // Cross variant
    table[0xBF] = PetASCII::Unicode(' '); // Space

    // PETSCII shifted characters (0xC0-0xDF) - Graphics in uppercase mode!
    // These map to screen codes 0x00-0x1F which access the graphics CHARACTER ROM
    // Based on actual C64 character ROM layout
    table[0xC0] = PetASCII::Unicode('─'); // horizontal line (top half missing)
    table[0xC1] = PetASCII::Unicode('│'); // vertical line (left half missing)
    table[0xC2] = PetASCII::Unicode('─'); // horizontal line (bottom half missing)
    table[0xC3] = PetASCII::Unicode('│'); // vertical line (right half missing)
    table[0xC4] = PetASCII::Unicode('╱'); // diagonal slash
    table[0xC5] = PetASCII::Unicode('╲'); // diagonal backslash
    table[0xC6] = PetASCII::Unicode('◢'); // triangle lower right
    table[0xC7] = PetASCII::Unicode('◣'); // triangle lower left
    table[0xC8] = PetASCII::Unicode('░'); // light checkerboard
    table[0xC9] = PetASCII::Unicode('▒'); // medium checkerboard
    table[0xCA] = PetASCII::Unicode('▌'); // left half block
    table[0xCB] = PetASCII::Unicode('┌'); // top-left corner
    table[0xCC] = PetASCII::Unicode('┐'); // top-right corner
    table[0xCD] = PetASCII::Unicode('└'); // bottom-left corner
    table[0xCE] = PetASCII::Unicode('┘'); // bottom-right corner
    table[0xCF] = PetASCII::Unicode('├'); // left T-junction
    table[0xD0] = PetASCII::Unicode('┤'); // right T-junction
    table[0xD1] = PetASCII::Unicode('┬'); // top T-junction
    table[0xD2] = PetASCII::Unicode('┴'); // bottom T-junction
    table[0xD3] = PetASCII::Unicode('┼'); // cross
    table[0xD4] = PetASCII::Unicode('●'); // filled circle
    table[0xD5] = PetASCII::Unicode('○'); // hollow circle
    table[0xD6] = PetASCII::Unicode('▐'); // right half block
    table[0xD7] = PetASCII::Unicode('▀'); // upper half block
    table[0xD8] = PetASCII::Unicode('▄'); // lower half block
    table[0xD9] = PetASCII::Unicode('█'); // full block
    table[0xDA] = PetASCII::Unicode('◤'); // triangle upper left
    table[0xDB] = PetASCII::Unicode('◥'); // triangle upper right
    table[0xDC] = PetASCII::Unicode('▗'); // quadrant lower right
    table[0xDD] = PetASCII::Unicode('▖'); // quadrant lower left
    table[0xDE] = PetASCII::Unicode('▝'); // quadrant upper right
    table[0xDF] = PetASCII::Unicode('▘'); // quadrant upper left

    // PETSCII 0xE0-0xFF - various graphics
    table[0xE0] = PetASCII::Unicode('░'); // Light shade
    table[0xE1] = PetASCII::Unicode('▒'); // Medium shade
    table[0xE2] = PetASCII::Unicode('▓'); // Dark shade
    table[0xE3] = PetASCII::Unicode('◆'); // Diamond
    table[0xE4] = PetASCII::Unicode('┼'); // Cross
    table[0xE5] = PetASCII::Unicode('◄'); // Left arrow
    table[0xE6] = PetASCII::Unicode('═'); // Double horizontal
    table[0xE7] = PetASCII::Unicode('►'); // Right arrow
    table[0xE8] = PetASCII::Unicode('?'); // Question mark
    for i in 0xE9..=0xFF {
        table[i] = PetASCII::Unicode('░'); // Various graphics -> light shade
    }

    table
}

/// Build the complete 256-entry PETSCII lookup table for lowercase/uppercase mode
pub fn build_petscii_table_lowercase() -> [PetASCII; 256] {
    let mut table = [PetASCII::Unicode('?'); 256];

    // Control codes (0x00-0x1F) - pass through for screen handling
    for i in 0x00..=0x1F {
        table[i] = PetASCII::Control(i as u8);
    }

    // Standard ASCII printable (0x20-0x5F)
    for i in 0x20..=0x5F {
        table[i] = PetASCII::Unicode(i as u8 as char);
    }

    // Same ASCII approximations as uppercase mode
    table[0x2D] = PetASCII::Unicode('─'); // '-' → horizontal line
    table[0x7C] = PetASCII::Unicode('│'); // '|' → vertical line
    table[0x2B] = PetASCII::Unicode('┼'); // '+' → cross
    table[0x23] = PetASCII::Unicode('█'); // '#' → full block

    // In lowercase mode, 0x41-0x5A are UPPERCASE letters (displayed as is)
    // Already handled by the loop above

    // PETSCII 0x60-0x7F - LOWERCASE letters in this mode
    for i in 0x61..=0x7A {
        // lowercase a-z
        table[i] = PetASCII::Unicode(i as u8 as char);
    }
    table[0x60] = PetASCII::Unicode('─'); // Still horizontal line
    table[0x7B] = PetASCII::Unicode('[');
    table[0x7C] = PetASCII::Unicode('│'); // Vertical line
    table[0x7D] = PetASCII::Unicode(']');
    table[0x7E] = PetASCII::Unicode('~');
    table[0x7F] = PetASCII::Unicode('·');

    // Control codes (0x80-0x9F) - colors and video control
    for i in 0x80..=0x9F {
        table[i] = PetASCII::Control(i as u8);
    }

    // PETSCII box drawing (0xA0-0xBF) - SAME as uppercase mode
    table[0xA0] = PetASCII::Unicode(' ');
    table[0xA1] = PetASCII::Unicode('▐');
    table[0xA2] = PetASCII::Unicode('░');
    table[0xA3] = PetASCII::Unicode('─');
    table[0xA4] = PetASCII::Unicode('▒');
    table[0xA5] = PetASCII::Unicode('▔');
    table[0xA6] = PetASCII::Unicode('▃');
    table[0xA7] = PetASCII::Unicode('▖');
    table[0xA8] = PetASCII::Unicode('▝');
    table[0xA9] = PetASCII::Unicode('┘');
    table[0xAA] = PetASCII::Unicode('▘');
    table[0xAB] = PetASCII::Unicode('╱');
    table[0xAC] = PetASCII::Unicode('▐');
    table[0xAD] = PetASCII::Unicode('╯');
    table[0xAE] = PetASCII::Unicode('╰');
    table[0xAF] = PetASCII::Unicode('╭');
    table[0xB0] = PetASCII::Unicode('╲');
    table[0xB1] = PetASCII::Unicode('├');
    table[0xB2] = PetASCII::Unicode('┤');
    table[0xB3] = PetASCII::Unicode('╮');
    table[0xB4] = PetASCII::Unicode('┬');
    table[0xB5] = PetASCII::Unicode('┴');
    table[0xB6] = PetASCII::Unicode('┼');
    table[0xB7] = PetASCII::Unicode('◆');
    table[0xB8] = PetASCII::Unicode('◇');
    table[0xB9] = PetASCII::Unicode('●');
    table[0xBA] = PetASCII::Unicode('○');
    table[0xBB] = PetASCII::Unicode('┼');
    table[0xBC] = PetASCII::Unicode('│');
    table[0xBD] = PetASCII::Unicode('│');
    table[0xBE] = PetASCII::Unicode('┼');
    table[0xBF] = PetASCII::Unicode(' ');

    // PETSCII shifted characters (0xC0-0xDF) - UPPERCASE in lowercase mode
    table[0xC0] = PetASCII::Unicode('─'); // Horizontal line
    for i in 0xC1..=0xDA {
        // Uppercase A-Z (these are the "shifted" versions in lowercase mode)
        table[i] = PetASCII::Unicode((i - 0xC0 + 0x41) as u8 as char);
    }
    table[0xDB] = PetASCII::Unicode('█');
    table[0xDC] = PetASCII::Unicode('▄');
    table[0xDD] = PetASCII::Unicode('│');
    table[0xDE] = PetASCII::Unicode('▐');
    table[0xDF] = PetASCII::Unicode('▄');

    // PETSCII 0xE0-0xFF - various graphics (same as uppercase mode)
    table[0xE0] = PetASCII::Unicode('░');
    table[0xE1] = PetASCII::Unicode('▒');
    table[0xE2] = PetASCII::Unicode('▓');
    table[0xE3] = PetASCII::Unicode('◆');
    table[0xE4] = PetASCII::Unicode('┼');
    table[0xE5] = PetASCII::Unicode('◄');
    table[0xE6] = PetASCII::Unicode('═');
    table[0xE7] = PetASCII::Unicode('►');
    table[0xE8] = PetASCII::Unicode('?');
    for i in 0xE9..=0xFF {
        table[i] = PetASCII::Unicode('░');
    }

    table
}

/// Get the appropriate PETSCII table for the current mode
pub fn get_petscii_table(mode: CharsetMode) -> [PetASCII; 256] {
    match mode {
        CharsetMode::UppercaseGraphics => build_petscii_table_uppercase(),
        CharsetMode::LowercaseUppercase => build_petscii_table_lowercase(),
    }
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
        0x60 => ('-', false),                                 // Horizontal line
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
        0xC0 => ('-', false),                                 // Horizontal line
        0xC1..=0xDA => ((byte - 0xC0 + 0x41) as char, false), // Shifted A-Z
        0xDB => ('#', false),                                 // Full block
        0xDC => ('_', false),
        0xDD => ('|', false), // Vertical line
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
    fn test_petscii_table_uppercase() {
        let table = build_petscii_table_uppercase();

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

        // Test uppercase letter in uppercase mode
        match table[0x41] {
            PetASCII::Unicode('A') => (),
            _ => panic!("Expected uppercase A"),
        }

        // Test graphics in range 0x61-0x7A (uppercase mode)
        match table[0x61] {
            PetASCII::Unicode('A') => (), // Maps to uppercase
            _ => panic!("Expected uppercase A from 0x61"),
        }
    }

    #[test]
    fn test_petscii_table_lowercase() {
        let table = build_petscii_table_lowercase();

        // Test lowercase letter in lowercase mode
        match table[0x61] {
            PetASCII::Unicode('a') => (),
            _ => panic!("Expected lowercase a"),
        }

        // Test uppercase letter still works
        match table[0x41] {
            PetASCII::Unicode('A') => (),
            _ => panic!("Expected uppercase A"),
        }

        // Graphics should still work
        match table[0xDD] {
            PetASCII::Unicode('│') => (),
            _ => panic!("Expected vertical line"),
        }
    }

    #[test]
    fn test_charset_mode() {
        let uppercase_table = get_petscii_table(CharsetMode::UppercaseGraphics);
        let lowercase_table = get_petscii_table(CharsetMode::LowercaseUppercase);

        // Verify 0x61 differs between modes
        match uppercase_table[0x61] {
            PetASCII::Unicode('A') => (), // Uppercase mode
            _ => panic!("Expected A in uppercase mode"),
        }

        match lowercase_table[0x61] {
            PetASCII::Unicode('a') => (), // Lowercase mode
            _ => panic!("Expected a in lowercase mode"),
        }
    }
}
