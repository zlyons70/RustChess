/// Holds board representation
#[derive(Debug)]
pub enum Square {
    // if a square is empty it doesn't need anything
    Empty,
    // if it is occupied, since we're using rust we cannot let it hold a piece
    // like in OOP, however it can hold the positon of the piece
    // so this holds an index to the piece in the pieces vector in the game struct
    Occupied(usize),
}
