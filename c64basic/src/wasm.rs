//! Browser front-end: a wasm-bindgen wrapper that runs the international
//! edition and renders the screen into an RGBA framebuffer by blitting 8x8
//! glyphs from the C64 character generator ROM, exactly as the VIC-II did.
//! The host JS blits the framebuffer to a canvas and forwards keypresses
//! as PETSCII bytes.

use crate::interp::Interp;
use crate::lang;
use crate::petscii::{screen_code, Charset, PALETTE};
use crate::screen::{Screen, COLS, ROWS};
use wasm_bindgen::prelude::*;

const CHARGEN: &[u8; 4096] = include_bytes!("../assets/chargen-901225-01.bin");
const LISTING: &str = include_str!("../../football-manager-intl.bas");

/// Full frame including border, like a real C64 display.
pub const FRAME_W: usize = 384;
pub const FRAME_H: usize = 272;
const ORIGIN_X: usize = (FRAME_W - COLS * 8) / 2;
const ORIGIN_Y: usize = (FRAME_H - ROWS * 8) / 2;

#[wasm_bindgen]
pub struct Machine {
    interp: Interp,
    frame: Vec<u8>,
}

#[wasm_bindgen]
impl Machine {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: Option<u32>) -> Result<Machine, JsValue> {
        let prog = lang::load_program(LISTING).map_err(|e| JsValue::from_str(&e))?;
        let interp = match seed {
            Some(s) => Interp::new_seeded(prog, Screen::new(), s as u64),
            None => Interp::new(prog, Screen::new()),
        }
        .map_err(|e| JsValue::from_str(&e))?;
        Ok(Machine {
            interp,
            frame: vec![0; FRAME_W * FRAME_H * 4],
        })
    }

    pub fn width(&self) -> usize {
        FRAME_W
    }

    pub fn height(&self) -> usize {
        FRAME_H
    }

    /// Execute up to `budget` BASIC statements. Returns true when the
    /// program has halted (END or runtime error).
    pub fn tick(&mut self, budget: u32) -> Result<bool, JsValue> {
        self.interp
            .run_slice(budget)
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Inject one PETSCII byte as a keypress.
    pub fn key(&mut self, b: u8) {
        self.interp.push_char(b);
    }

    /// Render the current screen into the RGBA framebuffer and return it.
    pub fn frame(&mut self) -> Vec<u8> {
        let s = &self.interp.screen;
        let rom_bank = match s.charset {
            Charset::UpperGraphics => 0usize,
            Charset::LowerUpper => 0x800,
        };
        let border = PALETTE[(s.border & 0x0F) as usize];
        let bg = PALETTE[(s.bg & 0x0F) as usize];

        for y in 0..FRAME_H {
            for x in 0..FRAME_W {
                let inside = (ORIGIN_X..ORIGIN_X + COLS * 8).contains(&x)
                    && (ORIGIN_Y..ORIGIN_Y + ROWS * 8).contains(&y);
                let rgb = if inside {
                    let cx = (x - ORIGIN_X) / 8;
                    let cy = (y - ORIGIN_Y) / 8;
                    let cell = s.cells[cy][cx];
                    let glyph = screen_code(cell.byte) as usize;
                    let row = CHARGEN[rom_bank + glyph * 8 + (y - ORIGIN_Y) % 8];
                    let mut bit = row >> (7 - (x - ORIGIN_X) % 8) & 1 == 1;
                    if cell.reverse {
                        bit = !bit;
                    }
                    if bit {
                        PALETTE[(cell.color & 0x0F) as usize]
                    } else {
                        bg
                    }
                } else {
                    border
                };
                let i = (y * FRAME_W + x) * 4;
                self.frame[i] = rgb[0];
                self.frame[i + 1] = rgb[1];
                self.frame[i + 2] = rgb[2];
                self.frame[i + 3] = 0xFF;
            }
        }
        self.frame.clone()
    }
}
