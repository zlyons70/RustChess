/// This module handles the game struct
pub use crate::board::Square;
use crate::pieces::{Color, Piece, PieceType};
use crate::constants::*;
pub struct Game{
    pieces: Vec<Piece>,
    squares: Vec<Square>,
    active_team: char,
    winner: bool,
    winner_team: String,
    fen: String,
    w_king_castle: bool,
    b_king_castle: bool,
    w_queen_castle: bool,
    b_queen_castle: bool,
    half_move: usize,
    full_move: usize,
}

// the goal of the to fn string is to take a board, and convert the board into a string
// and by board we me each square
// for each square we keep track of the index of the square and what piece it is
impl Game {
    fn push_square_and_piece(&mut self, piece_char: &char, index: &mut usize) {
        if piece_char.is_alphabetic() {
            let cur_piece_type = match piece_char.clone().to_ascii_lowercase() {
                'p' => PieceType::Pawn,
                'q' => PieceType::Queen,
                'k' => PieceType::King,
                'b' => PieceType::Bishop,
                'n' => PieceType::Knight,
                'r' => PieceType::Rook,
                _ => PieceType::Pawn
            };
            let mut cur_color = Color::Black;
            if piece_char.is_uppercase() {
                cur_color = Color::White;
            }
            let cur_piece = Piece {
                piece_type: cur_piece_type,
                color: cur_color,
                position: (1 as usize) << *index,
            };
            self.pieces.push(cur_piece);
            self.squares.push(Square::Occupied(index.clone()));
            *index += 1;
        } else {
            let empty_squares = piece_char.clone().to_digit(10).unwrap();
            for _i in 0..empty_squares {
                self.squares.push(Square::Empty);
            }
        }
    }

    pub fn initialize() -> Game {
        // Creates initail chess board state
        let mut game = Game {
            pieces: vec![],
            squares: vec![],
            active_team: 'w',
            winner: false,
            winner_team: "None".to_string(),
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            w_king_castle: true,
            b_king_castle: true,
            w_queen_castle: true,
            b_queen_castle: true,
            half_move: 0,
            full_move: 0,

        };
        let mut actual_index: usize = 0;
        for c in game.fen.clone().chars(){
            if c == ' '{
                break;
            }
            if c.is_ascii_alphanumeric() {
                game.push_square_and_piece(&c, &mut actual_index);
            }
        }
        game
    }

    pub fn to_string(&self) -> String {
        // used to create command line representation of current board
        println!("{:?}", self.squares);
        let mut board = "".to_string();
        let mut temp = "".to_string();
        for (i, square) in self.squares.iter().enumerate() {
            match square {
                Square::Empty => temp.push_str(&self.index_to_position(i)),
                Square::Occupied(idx) => temp.push_str(&self.pieces[*idx].to_string()),
            };
            if (i + 1) % 8 == 0 {
                temp.push_str("\n");
                board.push_str(&temp);
                temp.clear();
            }
        }
        board
    }
    // fn to_fn_string(&self) -> String {
    //     let mut board = "".to_owned();
    //     let mut temp = "".to_owned();
    //     let mut space_counter = 0;
    //     for(i, square) in self.squares.iter().enumerate() {
    //         if i % 8 {
    //             space_counter = 0;
    //         }
    //         match square {
    //             Empty => 
    //         }
    //     }
    // }

    pub fn bit_to_position(&self, bit: PiecePosition) -> Result<String, String> {
        if bit == 0 {
            Err("No Piece".to_string())
        } else {
            let bit_index = self.bit_scan(bit);
            Ok(self.index_to_position(bit_index))
        }
    }
    
    fn index_to_position(&self, index: usize) -> String {
        // this function takes an index and returns the position on the board
        let column = index % 8;
        let row = index / 8 + 1;
    
        format!("{}{}", COL_MAP[column], row)
    }
    
    fn bit_scan(&self,bit: PiecePosition) -> usize {
        let remainder: usize = bit % 67;
        MODTABLE[remainder]
    }
}