//! 40x25 PETSCII framebuffer with C64-style cursor, colours, reverse-video
//! and character-set switching. Renders to any `Write` using crossterm.

use crate::petscii::{c64_color, color_code_index, glyph, Charset};
use crossterm::{
    cursor,
    style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    QueueableCommand,
};
use std::io::{self, Write};

pub const COLS: usize = 40;
pub const ROWS: usize = 25;

#[derive(Copy, Clone)]
pub struct Cell {
    pub byte: u8,
    pub color: u8,
    pub reverse: bool,
}

impl Cell {
    const fn blank() -> Self {
        Cell {
            byte: 0x20,
            color: 14,
            reverse: false,
        }
    }
}

pub struct Screen {
    pub cells: [[Cell; COLS]; ROWS],
    pub row: usize,
    pub col: usize,
    pub color: u8,
    pub reverse: bool,
    pub bg: u8,
    pub border: u8,
    pub charset: Charset,
    dirty: bool,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::blank(); COLS]; ROWS],
            row: 0,
            col: 0,
            color: 14, // light blue (default C64 foreground)
            reverse: false,
            bg: 6,      // blue
            border: 14, // light blue
            charset: Charset::UpperGraphics,
            dirty: true,
        }
    }

    pub fn clear(&mut self) {
        for r in 0..ROWS {
            for c in 0..COLS {
                self.cells[r][c] = Cell {
                    byte: 0x20,
                    color: self.color,
                    reverse: false,
                };
            }
        }
        self.row = 0;
        self.col = 0;
        self.dirty = true;
    }

    /// Write a single PETSCII byte as PRINT would.
    /// Returns true if the byte was a carriage-return so that callers can
    /// optionally suppress the implicit newline after PRINT chains.
    pub fn put_byte(&mut self, b: u8) -> bool {
        match b {
            0x0D => {
                // Carriage return – newline, reset reverse, advance row.
                self.reverse = false;
                self.row += 1;
                self.col = 0;
                self.scroll_if_needed();
                self.dirty = true;
                true
            }
            0x05 | 0x1C | 0x1E | 0x1F | 0x81 | 0x90 | 0x95 | 0x96 | 0x97 | 0x98 | 0x99 | 0x9A
            | 0x9B | 0x9C | 0x9E | 0x9F => {
                if let Some(idx) = color_code_index(b) {
                    self.color = idx;
                }
                false
            }
            0x08 => false, // disable case-switching via SHIFT+CBM – no-op
            0x09 => false, // enable it – no-op
            0x0E => {
                self.charset = Charset::LowerUpper;
                self.dirty = true;
                false
            }
            0x8E => {
                self.charset = Charset::UpperGraphics;
                self.dirty = true;
                false
            }
            0x11 => {
                self.row = (self.row + 1).min(ROWS - 1);
                self.dirty = true;
                false
            }
            0x91 => {
                if self.row > 0 {
                    self.row -= 1;
                }
                self.dirty = true;
                false
            }
            0x1D => {
                self.col += 1;
                if self.col >= COLS {
                    self.col = 0;
                    self.row += 1;
                    self.scroll_if_needed();
                }
                self.dirty = true;
                false
            }
            0x9D => {
                if self.col > 0 {
                    self.col -= 1;
                } else if self.row > 0 {
                    self.row -= 1;
                    self.col = COLS - 1;
                }
                self.dirty = true;
                false
            }
            0x12 => {
                self.reverse = true;
                false
            }
            0x92 => {
                self.reverse = false;
                false
            }
            0x13 => {
                self.row = 0;
                self.col = 0;
                false
            }
            0x14 => {
                // Delete / backspace – remove char left of cursor.
                if self.col > 0 {
                    self.col -= 1;
                    self.cells[self.row][self.col] = Cell {
                        byte: 0x20,
                        color: self.color,
                        reverse: false,
                    };
                    self.dirty = true;
                }
                false
            }
            0x93 => {
                self.clear();
                false
            }
            0x94 => false, // insert – simplified
            _ => {
                // Printable or graphic byte – place on screen.
                if self.row >= ROWS {
                    self.row = ROWS - 1;
                    self.scroll_up();
                }
                self.cells[self.row][self.col] = Cell {
                    byte: b,
                    color: self.color,
                    reverse: self.reverse,
                };
                self.col += 1;
                if self.col >= COLS {
                    self.col = 0;
                    self.row += 1;
                    self.scroll_if_needed();
                }
                self.dirty = true;
                false
            }
        }
    }

    fn scroll_if_needed(&mut self) {
        if self.row >= ROWS {
            self.scroll_up();
            self.row = ROWS - 1;
        }
    }

    fn scroll_up(&mut self) {
        for r in 1..ROWS {
            self.cells[r - 1] = self.cells[r];
        }
        for c in 0..COLS {
            self.cells[ROWS - 1][c] = Cell {
                byte: 0x20,
                color: self.color,
                reverse: false,
            };
        }
    }

    /// Emulate `PRINT TAB(n)` – move the cursor to absolute column n (0-based),
    /// wrapping to next row if we would have to go backwards.
    pub fn print_tab(&mut self, n: usize) {
        let target = n % COLS;
        if target < self.col {
            self.row += 1;
            self.scroll_if_needed();
        }
        self.col = target;
        self.dirty = true;
    }

    pub fn set_border(&mut self, c: u8) {
        self.border = c & 0x0F;
        self.dirty = true;
    }

    pub fn set_bg(&mut self, c: u8) {
        self.bg = c & 0x0F;
        self.dirty = true;
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Render to stdout. Uses crossterm commands. Always redraws the full
    /// grid; with 40x25 cells this is cheap enough. `line_no` is displayed
    /// in the status row so the user can see which BASIC line is executing.
    pub fn render<W: Write>(&mut self, out: &mut W, line_no: u32) -> io::Result<()> {
        if !self.dirty {
            return Ok(());
        }
        out.queue(cursor::Hide)?.queue(cursor::MoveTo(0, 0))?;

        let bg_color = c64_color(self.bg);
        let border = c64_color(self.border);

        // Top border row (tiny – 1 line).
        out.queue(SetBackgroundColor(border))?
            .queue(SetForegroundColor(border))?
            .queue(Print(" ".repeat(COLS + 2)))?
            .queue(Print("\r\n"))?;

        for r in 0..ROWS {
            // Left border cell.
            out.queue(SetBackgroundColor(border))?.queue(Print(" "))?;

            for c in 0..COLS {
                let cell = self.cells[r][c];
                let fg = c64_color(cell.color);
                let (fg, bg) = if cell.reverse {
                    (bg_color, fg)
                } else {
                    (fg, bg_color)
                };
                out.queue(SetForegroundColor(fg))?
                    .queue(SetBackgroundColor(bg))?
                    .queue(Print(glyph(cell.byte, self.charset)))?;
            }

            // Right border cell.
            out.queue(SetBackgroundColor(border))?
                .queue(Print(" "))?
                .queue(Print("\r\n"))?;
        }

        // Bottom border row.
        out.queue(SetBackgroundColor(border))?
            .queue(SetForegroundColor(border))?
            .queue(Print(" ".repeat(COLS + 2)))?
            .queue(Print("\r\n"))?;

        out.queue(ResetColor)?
            .queue(Print(format!("  Ctrl-C to quit   line {}", line_no)))?
            .queue(Print("\r\n"))?;

        out.flush()?;
        self.dirty = false;
        Ok(())
    }
}
