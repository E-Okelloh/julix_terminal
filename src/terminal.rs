use vte::{Params, Parser, Perform};

pub struct Terminal {
    pub grid: Vec<Vec<Cell>>,
    pub cursor: Cursor,
    pub cols: usize,
    pub rows: usize,
    Parser: Parser,

}

#[derive(Clone, Debug)]
pub struct Cell {
    pub c: char,
    pub fg: Color,
    pub bg:Color,
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub struct Cursor{
    pub x: usize,
    pub y: usize,

}

impl Terminal {
    pub fn new(cols: usize, rows: usize) -> Self {
        let grid = Vec![vec![Cell::default(); cols]; rows];

        Self {
            grid,
            cursor: Cursor {x: 0, y: 0 },
            cols,
            rows,
            parser:Parser::new(),
        }
    }
}