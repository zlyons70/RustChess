use constants::{COL_MAP, MODTABLE, PiecePosition};
mod constants;
use game::Game;
mod game;
mod rayattacks;
pub mod board;
pub mod pieces;

fn main() {
    // instead of bevy use https://macroquad.rs/examples/
    let new_game = Game::initialize();
    println!("{}", new_game.to_string());
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let other = Game::fen_initialize(fen);
    println!("{}", other.to_string());
    // now that I have the initial game state set up I need to use bevy to actually display the board
}