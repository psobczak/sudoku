use std::{array, fmt::Display};

fn main() -> anyhow::Result<()> {
    let mut board = Board::<9>::new();
    board.set_cell(0, 0, 9)?;
    board.set_cell(0, 1, 9)?;
    board.set_cell(0, 2, 9)?;
    board.set_cell(1, 0, 9)?;
    board.set_cell(1, 1, 9)?;
    board.set_cell(1, 2, 9)?;
    board.set_cell(2, 0, 1)?;
    board.set_cell(2, 1, 1)?;
    board.set_cell(2, 2, 1)?;
    println!("{}", board);

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum SudokuError {
    #[error("cell with row {row} and column {col} was not found")]
    CellNotFound { row: usize, col: usize },
    #[error("cell value must be between 1 and 9 (was {0})")]
    CellValue(u8),
}

#[derive(Debug)]
pub struct Board<const SIZE: usize>([[Cell; SIZE]; SIZE]);

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Value(u8),
}

impl TryFrom<u8> for Cell {
    type Error = SudokuError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=9).contains(&value) {
            return Err(SudokuError::CellValue(value));
        }

        Ok(Cell::Value(value))
    }
}

impl<const SIZE: usize> Board<SIZE> {
    fn new() -> Self {
        Self(array::from_fn(|_| array::from_fn(|_| Cell::Empty)))
    }

    fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.0.get(row).and_then(|row| row.get(col))
    }

    fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.0.get_mut(row).and_then(|row| row.get_mut(col))
    }

    fn set_cell(&mut self, row: usize, col: usize, number: u8) -> Result<(), SudokuError> {
        self.get_cell_mut(row, col)
            .and_then(|c| {
                *c = Cell::try_from(number).unwrap();
                Some(())
            })
            .ok_or_else(|| SudokuError::CellValue(number))
    }
}

impl<const SIZE: usize> Display for Board<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            for cell in row {
                match cell {
                    Cell::Empty => write!(f, "_")?,
                    Cell::Value(val) => write!(f, "{}", val)?,
                };
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
