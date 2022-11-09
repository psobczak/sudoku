use std::{array, collections::HashSet, fmt::Display};

#[derive(Debug, thiserror::Error)]
pub enum SudokuError {
    #[error("cell value must be between 1 and 9 (was {0})")]
    CellValue(u8),
    #[error("input length must have 81 characters (was {0})")]
    InputLength(usize),
}

#[derive(Debug)]
pub struct Board([[Cell; 9]; 9]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Value(u8),
}

#[derive(Debug)]
pub enum Row {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

#[derive(Debug)]
pub enum Column {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<usize> for Row {
    fn from(value: usize) -> Self {
        match value {
            0 => Row::A,
            1 => Row::B,
            2 => Row::C,
            3 => Row::D,
            4 => Row::E,
            5 => Row::F,
            6 => Row::G,
            7 => Row::H,
            8 => Row::I,
            _ => todo!(),
        }
    }
}

impl From<Row> for usize {
    fn from(value: Row) -> Self {
        match value {
            Row::A => 0,
            Row::B => 1,
            Row::C => 2,
            Row::D => 3,
            Row::E => 4,
            Row::F => 5,
            Row::G => 6,
            Row::H => 7,
            Row::I => 8,
        }
    }
}

impl From<usize> for Column {
    fn from(value: usize) -> Self {
        match value {
            0 => Column::One,
            1 => Column::Two,
            2 => Column::Three,
            3 => Column::Four,
            4 => Column::Five,
            5 => Column::Six,
            6 => Column::Seven,
            7 => Column::Eight,
            8 => Column::Nine,
            _ => todo!(),
        }
    }
}

impl From<Column> for usize {
    fn from(value: Column) -> Self {
        match value {
            Column::One => 0,
            Column::Two => 1,
            Column::Three => 2,
            Column::Four => 3,
            Column::Five => 4,
            Column::Six => 5,
            Column::Seven => 6,
            Column::Eight => 7,
            Column::Nine => 8,
        }
    }
}

impl TryFrom<u8> for Cell {
    type Error = SudokuError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Cell::Empty),
            x if (1..=9).contains(&x) => Ok(Cell::Value(x)),
            _ => Err(SudokuError::CellValue(value)),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self(array::from_fn(|_| array::from_fn(|_| Cell::Empty)))
    }
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_cell_mut(&mut self, coordinates: (Row, Column)) -> Option<&mut Cell> {
        self.0
            .get_mut(usize::from(coordinates.0))
            .and_then(|row| row.get_mut(usize::from(coordinates.1)))
    }

    pub fn set_cell(&mut self, coordinates: (Row, Column), number: u8) -> Result<(), SudokuError> {
        self.get_cell_mut(coordinates)
            .and_then(|c| {
                *c = Cell::try_from(number).unwrap();
                Some(())
            })
            .ok_or_else(|| SudokuError::CellValue(number))
    }

    pub fn is_row_completed(&self, row: Row) -> bool {
        let index = usize::from(row);
        let mut set = HashSet::new();
        self.0[index].iter().all(|cell: &Cell| match cell {
            Cell::Empty => false,
            Cell::Value(num) => set.insert(num),
        })
    }

    pub fn is_column_completed(&self, col: Column) -> bool {
        let col: usize = col.into();
        let mut set = HashSet::new();
        self.0.iter().map(|row| row[col]).all(|cell| match cell {
            Cell::Empty => false,
            Cell::Value(num) => set.insert(num),
        })
    }

    pub fn all_rows_completed(&self) -> bool {
        (0..9).all(|row: usize| self.is_row_completed(row.into()))
    }

    pub fn all_columns_completed(&self) -> bool {
        (0..9).all(|column: usize| self.is_column_completed(column.into()))
    }

    pub fn get_square_of(&self, coordinate: (Row, Column)) -> [Cell; 9] {
        todo!()
    }

    pub fn get_row(&self, row: Row) -> [Cell; 9] {
        let index = usize::from(row);
        self.0[index]
    }

    pub fn get_column(&self, column: Column) -> [Cell; 9] {
        let index = usize::from(column);
        self.0
            .iter()
            .map(|row| row[index])
            .collect::<Vec<Cell>>()
            .try_into()
            .unwrap()
    }

    // pub fn get_units_of(&self, coordinate: (Row, Column)) -> [[Cell; 9]; 3] {
    //     self.0
    // }
}

impl TryFrom<&str> for Board {
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

impl Display for Board {
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
        let mut sudoku = Board::new();
        sudoku.set_cell((Row::A, Column::One), 1).unwrap();
        sudoku.set_cell((Row::A, Column::Two), 2).unwrap();
        sudoku.set_cell((Row::A, Column::Three), 3).unwrap();
        sudoku.set_cell((Row::A, Column::Four), 4).unwrap();
        sudoku.set_cell((Row::A, Column::Five), 5).unwrap();
        sudoku.set_cell((Row::A, Column::Six), 6).unwrap();
        sudoku.set_cell((Row::A, Column::Seven), 7).unwrap();
        sudoku.set_cell((Row::A, Column::Eight), 8).unwrap();
        sudoku.set_cell((Row::A, Column::Nine), 9).unwrap();
        assert!(sudoku.is_row_completed(Row::A))
    }

    #[test]
    fn row_contains_empty_cell() {
        let mut sudoku = Board::new();
        sudoku.set_cell((Row::A, Column::One), 1).unwrap();

        assert!(!sudoku.is_row_completed(Row::A))
    }

    #[test]
    fn row_contains_duplicates() {
        let mut sudoku = Board::new();
        sudoku.set_cell((Row::A, Column::One), 1).unwrap();
        sudoku.set_cell((Row::A, Column::Two), 1).unwrap();

        assert!(!sudoku.is_row_completed(Row::A))
    }

    #[test]
    fn column_completed() {
        let mut sudoku = Board::new();
        sudoku.set_cell((Row::A, Column::One), 1).unwrap();
        sudoku.set_cell((Row::B, Column::One), 2).unwrap();
        sudoku.set_cell((Row::C, Column::One), 3).unwrap();
        sudoku.set_cell((Row::D, Column::One), 4).unwrap();
        sudoku.set_cell((Row::E, Column::One), 5).unwrap();
        sudoku.set_cell((Row::F, Column::One), 6).unwrap();
        sudoku.set_cell((Row::G, Column::One), 7).unwrap();
        sudoku.set_cell((Row::H, Column::One), 8).unwrap();
        sudoku.set_cell((Row::I, Column::One), 9).unwrap();
        assert!(sudoku.is_column_completed(Column::One))
    }

    #[test]
    fn column_contains_empty_cell() {
        let mut sudoku = Board::new();
        sudoku.set_cell((Row::B, Column::Two), 1).unwrap();

        assert!(!sudoku.is_column_completed(Column::Two))
    }

    #[test]
    fn column_contains_duplicates() {
        let mut sudoku = Board::new();
        sudoku.set_cell((Row::A, Column::Two), 1).unwrap();
        sudoku.set_cell((Row::B, Column::Two), 1).unwrap();

        assert!(!sudoku.is_column_completed(Column::Two))
    }

    #[test]
    fn all_rows_and_columns_completed() {
        let input =
            "123456789578139624496872153952381467641297835387564291719623548864915372235748916";
        let board = Board::try_from(input).unwrap();

        assert!(board.all_rows_completed());
        assert!(board.all_columns_completed());
    }

    #[test]
    fn row_not_completed() {
        let input =
            "123456789578139624496872153952381467641297835387564291719623548864915372235748916";
        let mut board = Board::try_from(input).unwrap();

        board.set_cell((Row::A, Column::One), 1).unwrap();
        board.set_cell((Row::A, Column::Two), 1).unwrap();

        assert!(!board.all_rows_completed());
    }
}
