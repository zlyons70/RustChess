/// This module handles the game struct
pub use crate::board::Square;
use crate::pieces::{Color, Piece, PieceType};
use crate::constants::*;
pub struct Game{
    pieces: Vec<Piece>,
    squares: Vec<Square>,
    active_team: Color,
    winner: bool,
    winner_team: String,
    fen: String,
    en_passant: Option<PiecePosition>,
    w_king_castle: bool,
    b_king_castle: bool,
    w_queen_castle: bool,
    b_queen_castle: bool,
    half_move: usize,
    full_move: usize,
}


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
                _ => panic!("Invalid Piece")
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
       Game::fen_initialize("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())
    }

    pub fn fen_initialize(fen: String) -> Game {
        // This function takes a FEN string and creates a game struct
        let mut game = Game {
            pieces: vec![],
            squares: vec![],
            active_team: Color::White,
            winner: false,
            winner_team: "None".to_string(),
            fen: fen.clone(),
            w_king_castle: false,
            b_king_castle: false,
            w_queen_castle: false,
            b_queen_castle: false,
            half_move: 0,
            full_move: 0,
            en_passant: None,

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

        // once the game board is initialized we can use the split on function to 
        // look at the game state chars
        let (_, rest) = Game::split_on(&game.fen, ' ');
        let rest = rest.to_string();
        let (color, rest) = Game::split_on(&rest, ' ');
        match color {
            "w" => game.active_team = Color::White,
            "b" => game.active_team = Color::Black,
            _ => panic!("Non color")
        }
        // Do castleing rights
        let rest = rest.to_string();
        let (castle_rights, rest) = Game::split_on(&rest, ' ');
        for c in castle_rights.chars() {
            match c {
                'K' => game.w_king_castle = true,
                'Q' => game.w_queen_castle = true,
                'k' => game.b_king_castle = true,
                'q' => game.b_queen_castle = true,
                '-' => (),
                _ => panic!("castle rights error")
            }
        }
        // do en_passant
        let rest = rest.to_string();
        let (enpass, rest) = Game::split_on(&rest, ' ');
        match enpass {
            "-" => game.en_passant = None,
            s => game.en_passant = match Game::position_to_bit(s) {
                Err(e) => panic!("{}", e),
                Ok(bit) => Some(bit),
            },
        }
        // do halfmove clock
        let rest = rest.to_string();
        let (half, rest) = Game::split_on(&rest, ' ');
        match half.parse() {
            Ok(number) => game.half_move = number,
            Err(_) => panic!("not a number for half move"),
        }

        let rest = rest.to_string();
        let (full, _) = Game::split_on(&rest, ' ');
        match full.parse() {
            Ok(number) => game.full_move = number,
            Err(_) => panic!("not a number for half move"),
        }
        game
    }

    fn split_on(s: &String, split: char) -> (&str, &str) {
        for (i, c) in s.chars().enumerate() {
            if c == split {
                return (&s[0..i], &s[i+1..])
            }
        }
        (&s[..], "")
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

    pub fn to_fen(&self) -> String {
        // this function will take the game and return a FEN string
        todo!()
    }

    pub fn position_to_bit(pos: &str) -> Result<PiecePosition, String>
    {
        if pos.len() != 2 {
            return Err(format!("invalid len for enpass"))
        } else {
            let bytes = pos.as_bytes();
            let byte0 = bytes[0];
            if byte0 < 97 || byte0 >=97 + 8 {
                return Err(format!("invalid position in pos to byte"))
            } else {
                let column = (byte0 - 97) as u32;
                let byte1 = bytes[1];
                let row;
                match (byte1 as char).to_digit(10) {
                    Some(number) => if number < 1 || number > 8 {
                        return Err(format!("invalid position in byte: row"))
                    } else {
                        row = number - 1;
                    },
                    None => return Err(format!("invalid row char")),
                }
                let square_num = row * 8 + column;
                println!("yerp");
                println!("{}", square_num);
                let bit = (1 as usize) << square_num;
                Ok(bit)
            }
        }
    }

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