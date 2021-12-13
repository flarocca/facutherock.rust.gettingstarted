use crate::models::errors::TicTacToeError;
use std::fmt;

#[derive(Debug)]
pub struct Board {
    pub board: [[char; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [['-'; 3]; 3],
        }
    }

    fn is_cell_empty(&self, cell: &Cell) -> bool {
        // Validate
        self.board[cell.x][cell.y] == '-'
    }

    pub fn set_cell(&mut self, cell: &Cell, value: char) -> Result<(), TicTacToeError> {
        // Validate
        if self.is_cell_empty(&cell) != true {
            return Err(TicTacToeError::CellNotEmpty((*cell).clone()));
        }

        self.board[cell.x][cell.y] = value;

        Ok(())
    }

    pub fn game_won(&self) -> bool {
        // Horizontal
        (self.board[0][0] != '-' && self.board[0][0] == self.board[0][1] && self.board[0][1] == self.board[0][2]) ||
        (self.board[1][0] != '-' && self.board[1][0] == self.board[1][1] && self.board[1][1] == self.board[1][2]) ||
        (self.board[2][0] != '-' && self.board[2][0] == self.board[2][1] && self.board[2][1] == self.board[2][2]) ||
        // Vertical
        (self.board[0][0] != '-' && self.board[0][0] == self.board[1][0] && self.board[1][0] == self.board[2][0]) ||
        (self.board[0][1] != '-' && self.board[0][1] == self.board[1][1] && self.board[1][1] == self.board[2][1]) ||
        (self.board[0][2] != '-' && self.board[0][2] == self.board[1][2] && self.board[1][2] == self.board[2][2]) ||
        //Diagonal
        (self.board[0][0] != '-' && self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2]) ||
        (self.board[2][0] != '-' && self.board[2][0] == self.board[1][1] && self.board[1][1] == self.board[0][2])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn empty() -> Cell {
        Cell { x: 3, y: 3 }
    }

    pub fn is_empty(&self) -> bool {
        self.x == 3 || self.y == 3
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}
