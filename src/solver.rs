use crate::board::Board;

#[derive(Debug)]
pub struct Sudoku<'a> {
    sudoku: &'a Board,
}

impl<'a> Sudoku<'a> {
    pub fn new(sudoku: &'a mut Board) -> Self {
        Self { sudoku }
    }
}
