#![feature(iter_array_chunks)]

use std::{array, collections::HashSet, fmt::Display};

fn main() -> anyhow::Result<()> {
    let input = "123456789578139624496872153952381467641297835387564291719623548864915372235748916";
    let board = Sudoku::try_from(input)?;

    println!("{}", board);

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum SudokuError {
    #[error("cell with row {row} and column {col} was not found")]
    CellNotFound { row: usize, col: usize },
    #[error("cell value must be between 1 and 9 (was {0})")]
    CellValue(u8),
    #[error("input length must have 81 characters (was {0})")]
    InputLength(usize),
}

#[derive(Debug)]
pub struct Sudoku([[Cell; 9]; 9]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Default for Sudoku {
    fn default() -> Self {
        Self(array::from_fn(|_| array::from_fn(|_| Cell::Empty)))
    }
}

impl Sudoku {
    fn new() -> Self {
        Self::default()
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

    fn is_row_completed(&self, row: usize) -> bool {
        let mut set = HashSet::new();
        self.0[row].iter().all(|cell: &Cell| match cell {
            Cell::Empty => false,
            Cell::Value(num) => set.insert(num),
        })
    }

    fn is_column_completed(&self, col: usize) -> bool {
        let mut set = HashSet::new();
        self.0.iter().map(|row| row[col]).all(|cell| match cell {
            Cell::Empty => false,
            Cell::Value(num) => set.insert(num),
        })
    }

    fn all_rows_completed(&self) -> bool {
        (0..9).all(|row| self.is_row_completed(row))
    }

    fn all_columns_completed(&self) -> bool {
        (0..9).all(|column| self.is_column_completed(column))
    }
}

impl TryFrom<&str> for Sudoku {
    type Error = SudokuError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 81 {
            return Err(SudokuError::InputLength(value.len()));
        }

        let mut board: [[Cell; 9]; 9] = array::from_fn(|_| array::from_fn(|_| Cell::Empty));

        value
            .as_bytes()
            .chunks(9)
            .map(std::str::from_utf8)
            .map(|row| -> [Cell; 9] {
                row.unwrap()
                    .chars()
                    .map(|c| Cell::try_from(c.to_digit(10).unwrap() as u8).unwrap())
                    .collect::<Vec<Cell>>()
                    .try_into()
                    .unwrap()
            })
            .enumerate()
            .for_each(|(i, row)| board[i] = row);

        Ok(Self(board))
    }
}

impl Display for Sudoku {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_completed() {
        let mut sudoku = Sudoku::new();
        sudoku.set_cell(2, 0, 1).unwrap();
        sudoku.set_cell(2, 1, 2).unwrap();
        sudoku.set_cell(2, 2, 3).unwrap();
        sudoku.set_cell(2, 3, 4).unwrap();
        sudoku.set_cell(2, 4, 5).unwrap();
        sudoku.set_cell(2, 5, 6).unwrap();
        sudoku.set_cell(2, 6, 7).unwrap();
        sudoku.set_cell(2, 7, 8).unwrap();
        sudoku.set_cell(2, 8, 9).unwrap();
        assert!(sudoku.is_row_completed(2))
    }

    #[test]
    fn row_contains_empty_cell() {
        let mut sudoku = Sudoku::new();
        sudoku.set_cell(0, 0, 1).unwrap();

        assert!(!sudoku.is_row_completed(0))
    }

    #[test]
    fn row_contains_duplicates() {
        let mut sudoku = Sudoku::new();
        sudoku.set_cell(0, 0, 1).unwrap();
        sudoku.set_cell(0, 1, 1).unwrap();

        assert!(!sudoku.is_row_completed(0))
    }

    #[test]
    fn column_completed() {
        let mut sudoku = Sudoku::new();
        sudoku.set_cell(0, 1, 1).unwrap();
        sudoku.set_cell(1, 1, 2).unwrap();
        sudoku.set_cell(2, 1, 3).unwrap();
        sudoku.set_cell(3, 1, 4).unwrap();
        sudoku.set_cell(4, 1, 5).unwrap();
        sudoku.set_cell(5, 1, 6).unwrap();
        sudoku.set_cell(6, 1, 7).unwrap();
        sudoku.set_cell(7, 1, 8).unwrap();
        sudoku.set_cell(8, 1, 9).unwrap();
        assert!(sudoku.is_column_completed(1))
    }

    #[test]
    fn column_contains_empty_cell() {
        let mut sudoku = Sudoku::new();
        sudoku.set_cell(0, 0, 1).unwrap();

        assert!(!sudoku.is_column_completed(0))
    }

    #[test]
    fn column_contains_duplicates() {
        let mut sudoku = Sudoku::new();
        sudoku.set_cell(0, 0, 1).unwrap();
        sudoku.set_cell(1, 0, 1).unwrap();

        assert!(!sudoku.is_row_completed(0))
    }

    #[test]
    fn all_rows_and_columns_completed() {
        let input =
            "123456789578139624496872153952381467641297835387564291719623548864915372235748916";
        let board = Sudoku::try_from(input).unwrap();

        assert!(board.all_rows_completed());
        assert!(board.all_columns_completed());
    }

    #[test]
    fn row_not_completed() {
        let input =
            "123456789578139624496872153952381467641297835387564291719623548864915372235748916";
        let mut board = Sudoku::try_from(input).unwrap();

        board.set_cell(1, 1, 1).unwrap();
        board.set_cell(1, 2, 1).unwrap();

        assert!(!board.all_rows_completed());
    }
}
