use constants::{COL_MAP, MODTABLE, PiecePosition};
mod constants;
use game::Game;
mod game;
pub mod board;
pub mod pieces;


fn main() {
    let new_game = Game::initialize();
    println!("{}", new_game.to_string());
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let other = Game::fen_initialize(fen);
    println!("{}", other.to_string());
}
