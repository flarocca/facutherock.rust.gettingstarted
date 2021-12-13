mod models;
mod services;

use crate::models::game::Game;
use crate::services::renderer::PrintLnRenderer;
use crate::services::inputter::StdinInputter;

fn main() {
    println!("Welcome to Tic Tac Toe");

    let renderer = PrintLnRenderer::new();
    let inputter = StdinInputter::new();

    let mut game = Game::new('X', '0', &renderer, &inputter);
    
    game.start();
}