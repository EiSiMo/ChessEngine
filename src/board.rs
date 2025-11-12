use crate::square::Square;
use std::mem;
use crate::r#move::*;
use std::ops::Not;

pub const CASTLING_WK: u8 = 1;
pub const CASTLING_WK_MASK: u64 = 96; // F1 G1

pub const CASTLING_WQ: u8 = 2;
pub const CASTLING_WQ_MASK: u64 = 14; // B1 C1 D1

pub const CASTLING_BK: u8 = 4;
pub const CASTLING_BK_MASK: u64 = 6917529027641081856; // F8 G8

pub const CASTLING_BQ: u8 = 8;
pub const CASTLING_BQ_MASK: u64 = 1008806316530991104; // B8 C8 D8


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

pub struct Board {
    pub side_to_move: Color,
    
    pub pieces: [[u64; 2]; 6],
    
    pub occupied: [u64; 2],
    pub all_occupied: u64,
    pub empty_squares: u64,

    pub castling_rights: u8,
    pub en_passant_target: Option<Square>,

    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

impl Board {
    /// Creates a new Board instance from a FEN string.
    /// Assumes the FEN string is valid.
    pub fn from_fen(fen: &str) -> Self {
        let mut parts = fen.split_whitespace();

        // Initialisiere das 2D-Array
        let mut pieces = [[0u64; 2]; 6];
        let mut occupied = [0u64; 2];

        // Part 1: Piece placement
        let placement = parts.next().unwrap_or("");
        let mut rank = 7;
        let mut file = 0;

        for c in placement.chars() {
            if c.is_digit(10) {
                file += c.to_digit(10).unwrap_or(0) as usize;
            } else if c == '/' {
                rank -= 1;
                file = 0;
            } else if c.is_alphabetic() {
                let sq = (rank * 8 + file) as u8;
                let mask = 1u64 << sq;

                if c.is_uppercase() {
                    let color_idx = Color::White as usize;
                    occupied[color_idx] |= mask;
                    match c {
                        'P' => pieces[PieceType::Pawn as usize][color_idx] |= mask,
                        'N' => pieces[PieceType::Knight as usize][color_idx] |= mask,
                        'B' => pieces[PieceType::Bishop as usize][color_idx] |= mask,
                        'R' => pieces[PieceType::Rook as usize][color_idx] |= mask,
                        'Q' => pieces[PieceType::Queen as usize][color_idx] |= mask,
                        'K' => pieces[PieceType::King as usize][color_idx] |= mask,
                        _ => {}
                    }
                } else {
                    let color_idx = Color::Black as usize;
                    occupied[color_idx] |= mask;
                    match c {
                        'p' => pieces[PieceType::Pawn as usize][color_idx] |= mask,
                        'n' => pieces[PieceType::Knight as usize][color_idx] |= mask,
                        'b' => pieces[PieceType::Bishop as usize][color_idx] |= mask,
                        'r' => pieces[PieceType::Rook as usize][color_idx] |= mask,
                        'q' => pieces[PieceType::Queen as usize][color_idx] |= mask,
                        'k' => pieces[PieceType::King as usize][color_idx] |= mask,
                        _ => {}
                    }
                }
                file += 1;
            }
        }

        // Part 2: Active color
        let side_to_move = match parts.next().unwrap_or("w") {
            "b" => Color::Black,
            _ => Color::White,
        };

        // Part 3: Castling rights
        let mut castling_rights = 0u8;
        if let Some(castle_str) = parts.next() {
            if castle_str.contains('K') { castling_rights |= CASTLING_WK; }
            if castle_str.contains('Q') { castling_rights |= CASTLING_WQ; }
            if castle_str.contains('k') { castling_rights |= CASTLING_BK; }
            if castle_str.contains('q') { castling_rights |= CASTLING_BQ; }
        }

        // Part 4: En passant target
        let en_passant_target = match parts.next().unwrap_or("-") {
            "-" => None,
            sq_str => {
                let chars: Vec<char> = sq_str.chars().collect();
                let file = (chars[0] as u8 - b'a') as u8;
                let rank = (chars[1] as u8 - b'1') as u8;
                let sq_index = rank * 8 + file;
                // This is unsafe, but assumes the FEN is valid
                Some(unsafe { mem::transmute::<u8, Square>(sq_index) })
            }
        };

        // Part 5: Halfmove clock
        let halfmove_clock = parts.next().unwrap_or("0").parse::<u8>().unwrap_or(0);

        // Part 6: Fullmove number
        let fullmove_number = parts.next().unwrap_or("1").parse::<u16>().unwrap_or(1);

        let all_occupied = occupied[Color::White as usize] | occupied[Color::Black as usize];
        let empty_squares = !all_occupied;

        Board {
            side_to_move,
            pieces, // Geändertes Feld
            occupied,
            all_occupied,
            empty_squares,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
        }
    }

    /// Converts the current board state into a FEN string.
    pub fn to_fen(&self) -> String {
        let mut fen = String::with_capacity(90);

        // Part 1: Piece placement
        let mut empty_count = 0;
        for rank in (0..=7).rev() {
            for file in 0..=7 {
                let sq = (rank * 8 + file) as u8;
                let mask = 1u64 << sq;

                if let Some(piece) = self.get_piece_at(mask) {
                    if empty_count > 0 {
                        fen.push((b'0' + empty_count) as char);
                        empty_count = 0;
                    }
                    fen.push(piece);
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                fen.push((b'0' + empty_count) as char);
                empty_count = 0;
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        // Part 2: Active color
        fen.push(' ');
        fen.push(if self.side_to_move == Color::White { 'w' } else { 'b' });

        // Part 3: Castling rights
        fen.push(' ');
        let mut castle_str = String::new();
        if (self.castling_rights & CASTLING_WK) != 0 { castle_str.push('K'); }
        if (self.castling_rights & CASTLING_WQ) != 0 { castle_str.push('Q'); }
        if (self.castling_rights & CASTLING_BK) != 0 { castle_str.push('k'); }
        if (self.castling_rights & CASTLING_BQ) != 0 { castle_str.push('q'); }

        if castle_str.is_empty() {
            fen.push('-');
        } else {
            fen.push_str(&castle_str);
        }

        // Part 4: En passant target
        fen.push(' ');
        if let Some(sq) = self.en_passant_target {
            let sq_index = sq as u8;
            let file = (sq_index % 8) as u8;
            let rank = (sq_index / 8) as u8;
            fen.push((b'a' + file) as char);
            fen.push((b'1' + rank) as char);
        } else {
            fen.push('-');
        }

        // Part 5: Halfmove clock
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());

        // Part 6: Fullmove number
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }

    /// Helper function to find which piece (as a char) is on a given square mask.
    fn get_piece_at(&self, sq_mask: u64) -> Option<char> {
        let white = Color::White as usize;
        let black = Color::Black as usize;

        if (self.pieces[PieceType::Pawn as usize][white] & sq_mask) != 0 { return Some('P'); }
        if (self.pieces[PieceType::Pawn as usize][black] & sq_mask) != 0 { return Some('p'); }
        if (self.pieces[PieceType::Knight as usize][white] & sq_mask) != 0 { return Some('N'); }
        if (self.pieces[PieceType::Knight as usize][black] & sq_mask) != 0 { return Some('n'); }
        if (self.pieces[PieceType::Bishop as usize][white] & sq_mask) != 0 { return Some('B'); }
        if (self.pieces[PieceType::Bishop as usize][black] & sq_mask) != 0 { return Some('b'); }
        if (self.pieces[PieceType::Rook as usize][white] & sq_mask) != 0 { return Some('R'); }
        if (self.pieces[PieceType::Rook as usize][black] & sq_mask) != 0 { return Some('r'); }
        if (self.pieces[PieceType::Queen as usize][white] & sq_mask) != 0 { return Some('Q'); }
        if (self.pieces[PieceType::Queen as usize][black] & sq_mask) != 0 { return Some('q'); }
        if (self.pieces[PieceType::King as usize][white] & sq_mask) != 0 { return Some('K'); }
        if (self.pieces[PieceType::King as usize][black] & sq_mask) != 0 { return Some('k'); }

        None
    }

    /// Prints a single bitboard (u64) as an 8x8 grid for debugging.
    fn print_bitboard(&self, name: &str, bitboard: u64) {
        println!("--- {} ---", name);
        println!("  a b c d e f g h");
        for rank in (0..=7).rev() {
            print!("{} ", rank + 1);
            for file in 0..=7 {
                let sq_index = rank * 8 + file;
                let mask = 1u64 << sq_index;

                if (bitboard & mask) != 0 {
                    print!("1 ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
        println!("RAW VALUE: {}", bitboard);
        println!();
    }

    /// Prints all internal bitboards for debugging purposes.
    pub fn pretty_print_internals(&self) {
        println!("\n========= BOARD INTERNAL BITBOARDS =========");

        let white = Color::White as usize;
        let black = Color::Black as usize;

        self.print_bitboard("White Pawns", self.pieces[PieceType::Pawn as usize][white]);
        self.print_bitboard("Black Pawns", self.pieces[PieceType::Pawn as usize][black]);

        self.print_bitboard("White Knights", self.pieces[PieceType::Knight as usize][white]);
        self.print_bitboard("Black Knights", self.pieces[PieceType::Knight as usize][black]);

        self.print_bitboard("White Bishops", self.pieces[PieceType::Bishop as usize][white]);
        self.print_bitboard("Black Bishops", self.pieces[PieceType::Bishop as usize][black]);

        self.print_bitboard("White Rooks", self.pieces[PieceType::Rook as usize][white]);
        self.print_bitboard("Black Rooks", self.pieces[PieceType::Rook as usize][black]);

        self.print_bitboard("White Queens", self.pieces[PieceType::Queen as usize][white]);
        self.print_bitboard("Black Queens", self.pieces[PieceType::Queen as usize][black]);

        self.print_bitboard("White King", self.pieces[PieceType::King as usize][white]);
        self.print_bitboard("Black King", self.pieces[PieceType::King as usize][black]);

        println!("--- Aggregate Bitboards ---");
        self.print_bitboard("All White Pieces", self.occupied[white]);
        self.print_bitboard("All Black Pieces", self.occupied[black]);
        self.print_bitboard("All Occupied", self.all_occupied);
        self.print_bitboard("Empty Squares", self.empty_squares);

        println!("============================================\n");
    }
    
    pub fn make_move(&mut self, mv: Move) {
        let from = mv.value()& MOVE_FROM_MASK;
        let to = mv.value() & MOVE_TO_MASK;
        let flag = mv.value() & MOVE_FLAG_MASK;
        
        // promo must come first because of double usage of the flag bits
        if flag == MOVE_FLAG_NO_PROMO {
            
        } else { // 
            
        }
    }
}
