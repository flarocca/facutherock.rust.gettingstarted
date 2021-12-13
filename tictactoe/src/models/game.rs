use crate::models::board::Board;
use crate::models::board::Cell;
use crate::models::errors::TicTacToeError;
use crate::services::inputter::Inputter;
use crate::services::renderer::Renderer;

pub struct Game<'a> {

    player_one: char,
    player_two: char,
    current_player: char,
    board: Board,
    renderer: &'a dyn Renderer,
    inputter: &'a dyn Inputter,
}

impl<'a> Game<'a> {
    pub fn new(
        player_one: char,
        player_two: char,
        renderer: &'a dyn Renderer,
        inputter: &'a dyn Inputter,
    ) -> Game<'a> {
        let game = Game {
            player_one,
            player_two,
            renderer,
            inputter,
            current_player: player_one,
            board: Board::new(),
        };

        game.render_board();

        game
    }

    pub fn start(&mut self) {
        let mut cell = self.inputter.get_cell();
        let mut game_won = false;

        while cell.is_empty() == false && game_won == false {
            self.play(&mut cell); //handle result

            game_won = self.game_won();

            if game_won == false {
                cell = self.inputter.get_cell();
            }
        }
    }

    fn play(&mut self, cell: &Cell) -> Result<(), String> {
        let result = match self.board.set_cell(cell, self.current_player) {
            Ok(_) => Ok(()),
            Err(TicTacToeError::InvalidCell(desc)) => Err(desc.to_string()),
            Err(TicTacToeError::CellNotEmpty(desc)) => Err(desc.to_string()),
        };

        if self.current_player == self.player_one {
            self.current_player = self.player_two;
        } else {
            self.current_player = self.player_one;
        }

        self.render_board();

        result
    }

    pub fn game_won(&self) -> bool {
        self.board.game_won()
    }

    fn render_board(&self) {
        self.renderer.render("Tic Tac Toe Board");
        self.renderer.render("+-----+");
        self.renderer.render(&format!(
            "|{}|{}|{}|",
            self.board.board[0][0], self.board.board[0][1], self.board.board[0][2]
        ));
        self.renderer.render("+-+-+-+");
        self.renderer.render(
            &format!(
                "|{}|{}|{}|",
                self.board.board[1][0], self.board.board[1][1], self.board.board[1][2]
            )
            .to_owned(),
        );
        self.renderer.render("+-+-+-+");
        self.renderer.render(&format!(
            "|{}|{}|{}|",
            self.board.board[2][0], self.board.board[2][1], self.board.board[2][2]
        ));
        self.renderer.render("+-+-+-+");
    }
}
