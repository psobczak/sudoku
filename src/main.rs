mod board;
mod solver;

use board::Board;

fn main() -> anyhow::Result<()> {
    let input = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
    let board = Board::try_from(input)?;

    println!("{}", board);

    Ok(())
}
