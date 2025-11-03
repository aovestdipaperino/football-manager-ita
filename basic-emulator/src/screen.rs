use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::sync::{Arc, Mutex};

const SCREEN_WIDTH: usize = 40; // C64 screen width
const SCREEN_HEIGHT: usize = 25; // C64 screen height

#[derive(Clone)]
pub struct Screen {
    buffer: Arc<Mutex<Vec<Vec<char>>>>,
    cursor_x: Arc<Mutex<usize>>,
    cursor_y: Arc<Mutex<usize>>,
    border_color: Arc<Mutex<Color>>,
    background_color: Arc<Mutex<Color>>,
    reverse_mode: Arc<Mutex<bool>>,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            buffer: Arc::new(Mutex::new(vec![vec![' '; SCREEN_WIDTH]; SCREEN_HEIGHT])),
            cursor_x: Arc::new(Mutex::new(0)),
            cursor_y: Arc::new(Mutex::new(0)),
            border_color: Arc::new(Mutex::new(Color::Green)),
            background_color: Arc::new(Mutex::new(Color::Green)),
            reverse_mode: Arc::new(Mutex::new(false)),
        }
    }

    pub fn clear(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        *buffer = vec![vec![' '; SCREEN_WIDTH]; SCREEN_HEIGHT];
        *self.cursor_x.lock().unwrap() = 0;
        *self.cursor_y.lock().unwrap() = 0;
    }

    pub fn print(&self, text: &str) {
        let mut buffer = self.buffer.lock().unwrap();
        let mut x = *self.cursor_x.lock().unwrap();
        let mut y = *self.cursor_y.lock().unwrap();

        let mut chars = text.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\n' {
                y += 1;
                x = 0;
                if y >= SCREEN_HEIGHT {
                    // Scroll up
                    buffer.remove(0);
                    buffer.push(vec![' '; SCREEN_WIDTH]);
                    y = SCREEN_HEIGHT - 1;
                }
            } else if ch == '[' {
                // Parse placeholder
                let mut placeholder = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == ']' {
                        chars.next(); // consume ]
                        break;
                    }
                    placeholder.push(chars.next().unwrap());
                }

                // Handle known placeholders
                match placeholder.as_str() {
                    "CLR" => {
                        // Clear screen
                        *buffer = vec![vec![' '; SCREEN_WIDTH]; SCREEN_HEIGHT];
                        x = 0;
                        y = 0;
                    }
                    "REVERSE" => {
                        self.set_reverse_mode(true);
                    }
                    "SIDE" => {
                        if x < SCREEN_WIDTH {
                            buffer[y][x] = '│';
                            x += 1;
                        }
                    }
                    "BORDERS" => {
                        if x < SCREEN_WIDTH {
                            buffer[y][x] = '─';
                            x += 1;
                        }
                    }
                    "BALL" => {
                        if x < SCREEN_WIDTH {
                            buffer[y][x] = '●';
                            x += 1;
                        }
                    }
                    "FIELD" => {
                        if x < SCREEN_WIDTH {
                            buffer[y][x] = '▒';
                            x += 1;
                        }
                    }
                    _ => {
                        // Unknown placeholder - ignore
                    }
                }
            } else {
                let display_char = self.map_petscii(ch);

                if x < SCREEN_WIDTH {
                    buffer[y][x] = display_char;
                    x += 1;
                }

                if x >= SCREEN_WIDTH {
                    y += 1;
                    x = 0;
                    if y >= SCREEN_HEIGHT {
                        buffer.remove(0);
                        buffer.push(vec![' '; SCREEN_WIDTH]);
                        y = SCREEN_HEIGHT - 1;
                    }
                }
            }
        }

        *self.cursor_x.lock().unwrap() = x;
        *self.cursor_y.lock().unwrap() = y;
    }

    pub fn println(&self, text: &str) {
        self.print(text);
        self.print("\n");
    }

    pub fn set_border_color(&self, color_code: u8) {
        *self.border_color.lock().unwrap() = self.c64_color(color_code);
    }

    pub fn set_background_color(&self, color_code: u8) {
        *self.background_color.lock().unwrap() = self.c64_color(color_code);
    }

    pub fn set_reverse_mode(&self, enabled: bool) {
        *self.reverse_mode.lock().unwrap() = enabled;
    }

    pub fn get_content(&self) -> String {
        let buffer = self.buffer.lock().unwrap();
        buffer
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn tab_to(&self, column: usize) {
        let mut x = self.cursor_x.lock().unwrap();
        *x = column.min(SCREEN_WIDTH - 1);
    }

    fn c64_color(&self, code: u8) -> Color {
        match code {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Red,
            3 => Color::Cyan,
            4 => Color::Magenta,
            5 => Color::Green,
            6 => Color::Blue,
            7 => Color::Yellow,
            8 => Color::LightRed,
            9 => Color::Rgb(101, 67, 33), // Brown
            10 => Color::LightRed,
            11 => Color::DarkGray,
            12 => Color::Gray,
            13 => Color::LightGreen,
            14 => Color::LightBlue,
            15 => Color::LightCyan,
            _ => Color::White,
        }
    }

    fn map_petscii(&self, ch: char) -> char {
        // Map PETSCII graphics to Unicode box-drawing characters
        match ch {
            // Box drawing characters
            '│' => '│', // [SIDE] - vertical line (already Unicode)
            '─' => '─', // [BORDERS] - horizontal line (already Unicode)
            '┌' => '┌', // Top-left corner
            '┐' => '┐', // Top-right corner
            '└' => '└', // Bottom-left corner
            '┘' => '┘', // Bottom-right corner
            '├' => '├', // Left T-junction
            '┤' => '┤', // Right T-junction
            '┬' => '┬', // Top T-junction
            '┴' => '┴', // Bottom T-junction
            '┼' => '┼', // Cross

            // Special graphics
            '●' => '●', // [BALL] - soccer ball (already Unicode)
            '▒' => '▒', // [FIELD] - field/grass (already Unicode)
            '█' => '█', // Solid block
            '▓' => '▓', // Dark shade
            '░' => '░', // Light shade

            // PETSCII codes that need mapping
            '\u{00DD}' => '│', // PETSCII 221 - vertical line
            '\u{00A3}' => '─', // PETSCII 163 - horizontal line
            '\u{0051}' => '●', // PETSCII 81 - ball
            '\u{00A0}' => '▒', // PETSCII 160 - field

            // Pass through everything else
            _ => ch,
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        let size = f.area();

        // Create layout with 3-char borders
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(31), Constraint::Min(0)].as_ref())
            .split(size);

        let inner_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Length(46), Constraint::Min(0)].as_ref())
            .split(chunks[0]);

        let buffer = self.buffer.lock().unwrap();
        let border_color = *self.border_color.lock().unwrap();
        let background_color = *self.background_color.lock().unwrap();

        // Convert buffer to text lines
        let lines: Vec<Line> = buffer
            .iter()
            .map(|row| Line::from(row.iter().collect::<String>()))
            .collect();

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().bg(background_color)),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(paragraph, inner_chunks[0]);
    }
}
