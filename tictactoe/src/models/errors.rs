use std::{error::Error, fmt};

use crate::models::board::Cell;

#[derive(Debug)]
pub enum TicTacToeError {
    InvalidCell(Cell),
    CellNotEmpty(Cell),
}

impl Error for TicTacToeError {}

impl fmt::Display for TicTacToeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TicTacToeError::InvalidCell(cell) => {
                write!(f, "Cell {} is not part of the board", cell)
            }
            TicTacToeError::CellNotEmpty(cell) => write!(f, "Cell {} is not empty", cell),
        }
    }
}
