use constants::{COL_MAP, MODTABLE, PiecePosition};
mod constants;
use game::Game;
mod game;
pub mod board;
pub mod pieces;


fn main() {
    let new_game = Game::initialize();
    println!("{}", new_game.to_string())
}
