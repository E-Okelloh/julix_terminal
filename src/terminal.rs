use vte::{Params, Parser, Perform};

pub struct Terminal {
    pub grid: Vec<Vec<Cell>>,
    pub cursor: Cursor,
    pub cols: usize,
    pub rows: usize,
    parser: Parser,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub c: char,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Terminal {
    pub fn new(cols: usize, rows: usize) -> Self {
        let grid = vec![vec![Cell::default(); cols]; rows];
        
        Self {
            grid,
            cursor: Cursor { x: 0, y: 0 },
            cols,
            rows,
            parser: Parser::new(),
        }
    }

    pub fn feed_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.advance_byte(*byte);
        }
    }

    fn advance_byte(&mut self, byte: u8) {
        // Temporarily take ownership of the parser to avoid borrow checker issues
        let mut parser = std::mem::replace(&mut self.parser, Parser::new());
        
        let mut performer = TerminalPerformer {
            terminal: self,
        };
        
        parser.advance(&mut performer, byte);
        
        // Put the parser back
        self.parser = parser;
    }

    fn write_char(&mut self, c: char) {
        if self.cursor.x >= self.cols {
            self.cursor.x = 0;
            self.cursor.y += 1;
        }
        
        if self.cursor.y >= self.rows {
            self.scroll_up(1);
            self.cursor.y = self.rows - 1;
        }

        self.grid[self.cursor.y][self.cursor.x].c = c;
        self.cursor.x += 1;
    }

    fn scroll_up(&mut self, lines: usize) {
        self.grid.drain(0..lines);
        for _ in 0..lines {
            self.grid.push(vec![Cell::default(); self.cols]);
        }
    }

    pub fn resize(&mut self, cols: usize, rows: usize) {
        self.cols = cols;
        self.rows = rows;
        
        self.grid.resize(rows, vec![Cell::default(); cols]);
        for row in &mut self.grid {
            row.resize(cols, Cell::default());
        }
        
        self.cursor.x = self.cursor.x.min(cols.saturating_sub(1));
        self.cursor.y = self.cursor.y.min(rows.saturating_sub(1));
    }
}

struct TerminalPerformer<'a> {
    terminal: &'a mut Terminal,
}

impl<'a> Perform for TerminalPerformer<'a> {
    fn print(&mut self, c: char) {
        self.terminal.write_char(c);
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.terminal.cursor.y += 1;
                if self.terminal.cursor.y >= self.terminal.rows {
                    self.terminal.scroll_up(1);
                    self.terminal.cursor.y = self.terminal.rows - 1;
                }
            }
            b'\r' => self.terminal.cursor.x = 0,
            b'\x08' => {
                if self.terminal.cursor.x > 0 {
                    self.terminal.cursor.x -= 1;
                }
            }
            b'\t' => {
                // Tab: move to next 8-column boundary
                let next_tab = (self.terminal.cursor.x / 8 + 1) * 8;
                self.terminal.cursor.x = next_tab.min(self.terminal.cols - 1);
            }
            _ => {}
        }
    }

    fn csi_dispatch(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, action: char) {
        match action {
            'H' | 'f' => {
                // Cursor position
                let y = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(1) - 1;
                let x = params.iter().nth(1).and_then(|p| p.first()).copied().unwrap_or(1) - 1;
                self.terminal.cursor.x = (x as usize).min(self.terminal.cols.saturating_sub(1));
                self.terminal.cursor.y = (y as usize).min(self.terminal.rows.saturating_sub(1));
            }
            'A' => {
                // Cursor up
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(1);
                self.terminal.cursor.y = self.terminal.cursor.y.saturating_sub(n as usize);
            }
            'B' => {
                // Cursor down
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(1);
                self.terminal.cursor.y = (self.terminal.cursor.y + n as usize).min(self.terminal.rows.saturating_sub(1));
            }
            'C' => {
                // Cursor forward
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(1);
                self.terminal.cursor.x = (self.terminal.cursor.x + n as usize).min(self.terminal.cols.saturating_sub(1));
            }
            'D' => {
                // Cursor backward
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(1);
                self.terminal.cursor.x = self.terminal.cursor.x.saturating_sub(n as usize);
            }
            'J' => {
                // Clear screen
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(0);
                match n {
                    0 => {
                        // Clear from cursor to end of screen
                        for y in self.terminal.cursor.y..self.terminal.rows {
                            for x in 0..self.terminal.cols {
                                if y == self.terminal.cursor.y && x < self.terminal.cursor.x {
                                    continue;
                                }
                                self.terminal.grid[y][x] = Cell::default();
                            }
                        }
                    }
                    2 => {
                        // Clear entire screen
                        for row in &mut self.terminal.grid {
                            for cell in row {
                                *cell = Cell::default();
                            }
                        }
                    }
                    _ => {}
                }
            }
            'K' => {
                // Clear line
                let n = params.iter().next().and_then(|p| p.first()).copied().unwrap_or(0);
                let y = self.terminal.cursor.y;
                match n {
                    0 => {
                        // Clear from cursor to end of line
                        for x in self.terminal.cursor.x..self.terminal.cols {
                            self.terminal.grid[y][x] = Cell::default();
                        }
                    }
                    2 => {
                        // Clear entire line
                        for x in 0..self.terminal.cols {
                            self.terminal.grid[y][x] = Cell::default();
                        }
                    }
                    _ => {}
                }
            }
            'm' => {
                // SGR - Set Graphics Rendition (colors, bold, etc.)
                // For now, we'll ignore this and use default colors
                // You can implement color parsing here later
            }
            _ => {}
        }
    }

    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: char) {}
    fn put(&mut self, _byte: u8) {}
    fn unhook(&mut self) {}
    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}
    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {}
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            c: ' ',
            fg: Color { r: 255, g: 255, b: 255 },
            bg: Color { r: 0, g: 0, b: 0 },
        }
    }
}