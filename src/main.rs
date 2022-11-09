mod solver;
mod board;

use solver::Sudoku;
use board::Board;
use board::Row;

fn main() -> anyhow::Result<()> {
    let input = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
    let board = Board::try_from(input)?;

    let xd: Row = 1_usize.into();
    let dx = Row::from(2_usize);

    let dddd: usize = usize::from(Row::B);

    Ok(())
}
