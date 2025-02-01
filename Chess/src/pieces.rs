use crate::constants::PiecePosition;

#[derive(Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub position: PiecePosition,
    pub color: Color,
}
#[derive(Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}
#[derive(Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl Piece {
    pub fn to_string(&self) -> String {
    // each piece should be able to return a string version of itself
    let mut result = match self.piece_type {
        PieceType::Pawn => "p",
        PieceType::Bishop => "b",
        PieceType::Knight => "n",
        PieceType::King => "k",
        PieceType::Queen => "q",
        PieceType::Rook => "r"
    }.to_string();
    if self.color == Color::White {
        result = result.to_ascii_uppercase();
    }
    result

    }
}