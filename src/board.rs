use crate::r#move::*;
use crate::square::{Square, SQUARES};
use std::mem;
use std::ops::Not;

pub const CASTLING_WK_FLAG: u8 = 1;
pub const CASTLING_WK_MASK: u64 = 96; // F1 G1
pub const CASTLING_WK_K_POS_MASK: u64 = 16; // E1
pub const CASTLING_WK_R_POS_MASK: u64 = 128; // H1

pub const CASTLING_WQ_FLAG: u8 = 2;
pub const CASTLING_WQ_MASK: u64 = 14; // B1 C1 D1
pub const CASTLING_WQ_K_POS_MASK: u64 = 16; // E1
pub const CASTLING_WQ_R_POS_MASK: u64 = 1; // A1

pub const CASTLING_BK_FLAG: u8 = 4;
pub const CASTLING_BK_MASK: u64 = 6917529027641081856; // F8 G8
pub const CASTLING_BK_K_POS_MASK: u64 = 1152921504606846976; // E8
pub const CASTLING_BK_R_POS_MASK: u64 = 9223372036854775808; // H8

pub const CASTLING_BQ_FLAG: u8 = 8;
pub const CASTLING_BQ_MASK: u64 = 1008806316530991104; // B8 C8 D8
pub const CASTLING_BQ_K_POS_MASK: u64 = 1152921504606846976; // E8
pub const CASTLING_BQ_R_POS_MASK: u64 = 72057594037927936; // A8

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

pub const PIECE_TYPES: [PieceType; 6] = [
    PieceType::Pawn,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Rook,
    PieceType::Queen,
    PieceType::King,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub side_to_move: Color,

    pub pieces: [[u64; 2]; 6],
    pub pieces_on_squares: [Option<PieceType>; 64], // <-- ADDED

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

        // Initialisiere die Arrays
        let mut pieces = [[0u64; 2]; 6];
        let mut occupied = [0u64; 2];
        let mut pieces_on_squares = [None; 64]; // <-- ADDED

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
                        'P' => {
                            pieces[PieceType::Pawn as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Pawn); // <-- ADDED
                        }
                        'N' => {
                            pieces[PieceType::Knight as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Knight); // <-- ADDED
                        }
                        'B' => {
                            pieces[PieceType::Bishop as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Bishop); // <-- ADDED
                        }
                        'R' => {
                            pieces[PieceType::Rook as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Rook); // <-- ADDED
                        }
                        'Q' => {
                            pieces[PieceType::Queen as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Queen); // <-- ADDED
                        }
                        'K' => {
                            pieces[PieceType::King as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::King); // <-- ADDED
                        }
                        _ => {}
                    }
                } else {
                    let color_idx = Color::Black as usize;
                    occupied[color_idx] |= mask;
                    match c {
                        'p' => {
                            pieces[PieceType::Pawn as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Pawn); // <-- ADDED
                        }
                        'n' => {
                            pieces[PieceType::Knight as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Knight); // <-- ADDED
                        }
                        'b' => {
                            pieces[PieceType::Bishop as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Bishop); // <-- ADDED
                        }
                        'r' => {
                            pieces[PieceType::Rook as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Rook); // <-- ADDED
                        }
                        'q' => {
                            pieces[PieceType::Queen as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Queen); // <-- ADDED
                        }
                        'k' => {
                            pieces[PieceType::King as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::King); // <-- ADDED
                        }
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
            if castle_str.contains('K') {
                castling_rights |= CASTLING_WK_FLAG;
            }
            if castle_str.contains('Q') {
                castling_rights |= CASTLING_WQ_FLAG;
            }
            if castle_str.contains('k') {
                castling_rights |= CASTLING_BK_FLAG;
            }
            if castle_str.contains('q') {
                castling_rights |= CASTLING_BQ_FLAG;
            }
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
            pieces,
            pieces_on_squares, // <-- ADDED
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
        fen.push(if self.side_to_move == Color::White {
            'w'
        } else {
            'b'
        });

        // Part 3: Castling rights
        fen.push(' ');
        let mut castle_str = String::new();
        if (self.castling_rights & CASTLING_WK_FLAG) != 0 {
            castle_str.push('K');
        }
        if (self.castling_rights & CASTLING_WQ_FLAG) != 0 {
            castle_str.push('Q');
        }
        if (self.castling_rights & CASTLING_BK_FLAG) != 0 {
            castle_str.push('k');
        }
        if (self.castling_rights & CASTLING_BQ_FLAG) != 0 {
            castle_str.push('q');
        }

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

        if (self.pieces[PieceType::Pawn as usize][white] & sq_mask) != 0 {
            return Some('P');
        }
        if (self.pieces[PieceType::Pawn as usize][black] & sq_mask) != 0 {
            return Some('p');
        }
        if (self.pieces[PieceType::Knight as usize][white] & sq_mask) != 0 {
            return Some('N');
        }
        if (self.pieces[PieceType::Knight as usize][black] & sq_mask) != 0 {
            return Some('n');
        }
        if (self.pieces[PieceType::Bishop as usize][white] & sq_mask) != 0 {
            return Some('B');
        }
        if (self.pieces[PieceType::Bishop as usize][black] & sq_mask) != 0 {
            return Some('b');
        }
        if (self.pieces[PieceType::Rook as usize][white] & sq_mask) != 0 {
            return Some('R');
        }
        if (self.pieces[PieceType::Rook as usize][black] & sq_mask) != 0 {
            return Some('r');
        }
        if (self.pieces[PieceType::Queen as usize][white] & sq_mask) != 0 {
            return Some('Q');
        }
        if (self.pieces[PieceType::Queen as usize][black] & sq_mask) != 0 {
            return Some('q');
        }
        if (self.pieces[PieceType::King as usize][white] & sq_mask) != 0 {
            return Some('K');
        }
        if (self.pieces[PieceType::King as usize][black] & sq_mask) != 0 {
            return Some('k');
        }

        None
    }

    /// Prints the board as a human-readable ASCII grid.
    pub fn pretty_print_ascii(&self) {
        println!("\n   a b c d e f g h");
        for rank in (0..=7).rev() {
            print!("{}  ", rank + 1); // Rank annotation
            for file in 0..=7 {
                let sq = (rank * 8 + file) as u8;
                let mask = 1u64 << sq;

                if let Some(piece) = self.get_piece_at(mask) {
                    print!("{} ", piece);
                } else {
                    print!(". ");
                }
            }
            println!(" {}", rank + 1); // Rank annotation
        }
        println!("   a b c d e f g h\n");
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

        self.print_bitboard(
            "White Knights",
            self.pieces[PieceType::Knight as usize][white],
        );
        self.print_bitboard(
            "Black Knights",
            self.pieces[PieceType::Knight as usize][black],
        );

        self.print_bitboard(
            "White Bishops",
            self.pieces[PieceType::Bishop as usize][white],
        );
        self.print_bitboard(
            "Black Bishops",
            self.pieces[PieceType::Bishop as usize][black],
        );

        self.print_bitboard("White Rooks", self.pieces[PieceType::Rook as usize][white]);
        self.print_bitboard("Black Rooks", self.pieces[PieceType::Rook as usize][black]);

        self.print_bitboard(
            "White Queens",
            self.pieces[PieceType::Queen as usize][white],
        );
        self.print_bitboard(
            "Black Queens",
            self.pieces[PieceType::Queen as usize][black],
        );

        self.print_bitboard("White King", self.pieces[PieceType::King as usize][white]);
        self.print_bitboard("Black King", self.pieces[PieceType::King as usize][black]);

        println!("--- Aggregate Bitboards ---");
        self.print_bitboard("All White Pieces", self.occupied[white]);
        self.print_bitboard("All Black Pieces", self.occupied[black]);
        self.print_bitboard("All Occupied", self.all_occupied);
        self.print_bitboard("Empty Squares", self.empty_squares);

        println!("============================================\n");
    }

    fn clear_square(&mut self, target_square: Square, color: Color) -> PieceType {
        let target_square_bitboard = target_square.to_bitboard();

        // update occupancy helper bitboards
        self.occupied[color as usize] ^= target_square_bitboard;
        self.all_occupied ^= target_square_bitboard;
        self.empty_squares |= target_square_bitboard;

        self.pieces_on_squares[target_square as usize] = None; // <-- ADDED

        for piece_type in PIECE_TYPES {
            if self.pieces[piece_type as usize][color as usize] & target_square_bitboard > 0 {
                self.pieces[piece_type as usize][color as usize] ^= target_square_bitboard;
                return piece_type;
            }
        }
        panic!("fn 'clear_square' failed: no piece found");
    }

    fn remove_specific_piece(
        &mut self,
        target_square: Square,
        color: Color,
        piece_type: PieceType,
    ) {
        let target_square_bitboard = target_square.to_bitboard();
        self.pieces[piece_type as usize][color as usize] ^= target_square_bitboard;

        self.pieces_on_squares[target_square as usize] = None; // <-- ADDED

        // update occupancy helper bitboards
        self.occupied[color as usize] ^= target_square_bitboard;
        self.all_occupied ^= target_square_bitboard;
        self.empty_squares |= target_square_bitboard;
    }

    fn put_piece(&mut self, target_square: Square, color: Color, piece_type: PieceType) {
        let target_square_bitboard = target_square.to_bitboard();
        self.pieces[piece_type as usize][color as usize] |= target_square_bitboard;

        self.pieces_on_squares[target_square as usize] = Some(piece_type); // <-- ADDED

        // update occupancy helper bitboards
        self.occupied[color as usize] |= target_square_bitboard;
        self.all_occupied |= target_square_bitboard;
        self.empty_squares ^= target_square_bitboard;
    }

    pub fn make_move(&mut self, mv: Move) -> UndoMove {
        let from = mv.get_from();
        let to = mv.get_to();
        let flags = mv.get_flags();

        let old_en_passant_target: Option<Square> = self.en_passant_target;
        let old_castling_rights: u8 = self.castling_rights;
        let old_halfmove_clock: u8 = self.halfmove_clock;

        // needed for half move tracking
        let old_friendly_pawns = self.pieces[PieceType::Pawn as usize][self.side_to_move as usize];
        let old_total_pieces = self.all_occupied.count_ones();

        let mut opt_captured_piece: Option<PieceType> = None;
        let mut opt_en_passant_target: Option<Square> = None;

        match flags {
            MOVE_FLAG_QUIET => {
                let piece_type_from = self.clear_square(from, self.side_to_move);
                self.put_piece(to, self.side_to_move, piece_type_from);
            }
            MOVE_FLAG_CAPTURE => {
                let piece_type_from = self.clear_square(from, self.side_to_move);
                opt_captured_piece = Some(self.clear_square(to, !self.side_to_move));
                self.put_piece(to, self.side_to_move, piece_type_from);
            }
            MOVE_FLAG_DOUBLE_PAWN => {
                let piece_type_from = self.clear_square(from, self.side_to_move);
                self.put_piece(to, self.side_to_move, piece_type_from);
                opt_en_passant_target = Some(to + (self.side_to_move as i8 * 16 - 8));
            }
            MOVE_FLAG_PROMO_Q => {
                self.remove_specific_piece(from, self.side_to_move, PieceType::Pawn);
                self.put_piece(to, self.side_to_move, PieceType::Queen);
            }
            MOVE_FLAG_PROMO_N => {
                self.remove_specific_piece(from, self.side_to_move, PieceType::Pawn);
                self.put_piece(to, self.side_to_move, PieceType::Knight);
            }
            MOVE_FLAG_PROMO_B => {
                self.remove_specific_piece(from, self.side_to_move, PieceType::Pawn);
                self.put_piece(to, self.side_to_move, PieceType::Bishop);
            }
            MOVE_FLAG_PROMO_R => {
                self.remove_specific_piece(from, self.side_to_move, PieceType::Pawn);
                self.put_piece(to, self.side_to_move, PieceType::Rook);
            }
            MOVE_FLAG_PROMO_Q_CAP => {
                self.remove_specific_piece(from, self.side_to_move, PieceType::Pawn);
                opt_captured_piece = Some(self.clear_square(to, !self.side_to_move));
                self.put_piece(to, self.side_to_move, PieceType::Queen);
            }
            MOVE_FLAG_PROMO_N_CAP => {
                self.remove_specific_piece(from, self.side_to_move, PieceType::Pawn);
                opt_captured_piece = Some(self.clear_square(to, !self.side_to_move));
                self.put_piece(to, self.side_to_move, PieceType::Knight);
            }
            MOVE_FLAG_PROMO_B_CAP => {
                self.remove_specific_piece(from, self.side_to_move, PieceType::Pawn);
                opt_captured_piece = Some(self.clear_square(to, !self.side_to_move));
                self.put_piece(to, self.side_to_move, PieceType::Bishop);
            }
            MOVE_FLAG_PROMO_R_CAP => {
                self.remove_specific_piece(from, self.side_to_move, PieceType::Pawn);
                opt_captured_piece = Some(self.clear_square(to, !self.side_to_move));
                self.put_piece(to, self.side_to_move, PieceType::Rook);
            }
            MOVE_FLAG_WK_CASTLE => {
                self.remove_specific_piece(Square::E1, Color::White, PieceType::King);
                self.remove_specific_piece(Square::H1, Color::White, PieceType::Rook);
                self.put_piece(Square::G1, Color::White, PieceType::King);
                self.put_piece(Square::F1, Color::White, PieceType::Rook);
            }
            MOVE_FLAG_BK_CASTLE => {
                self.remove_specific_piece(Square::E8, Color::Black, PieceType::King);
                self.remove_specific_piece(Square::H8, Color::Black, PieceType::Rook);
                self.put_piece(Square::G8, Color::Black, PieceType::King);
                self.put_piece(Square::F8, Color::Black, PieceType::Rook);
            }
            MOVE_FLAG_WQ_CASTLE => {
                self.remove_specific_piece(Square::E1, Color::White, PieceType::King);
                self.remove_specific_piece(Square::A1, Color::White, PieceType::Rook);
                self.put_piece(Square::C1, Color::White, PieceType::King);
                self.put_piece(Square::D1, Color::White, PieceType::Rook);
            }
            MOVE_FLAG_BQ_CASTLE => {
                self.remove_specific_piece(Square::E8, Color::Black, PieceType::King);
                self.remove_specific_piece(Square::A8, Color::Black, PieceType::Rook);
                self.put_piece(Square::C8, Color::Black, PieceType::King);
                self.put_piece(Square::D8, Color::Black, PieceType::Rook);
            }
            MOVE_FLAG_EN_PASSANT => {
                self.remove_specific_piece(from, self.side_to_move, PieceType::Pawn);
                if self.side_to_move == Color::White {
                    self.remove_specific_piece(to - 8_u8, !self.side_to_move, PieceType::Pawn);
                } else {
                    self.remove_specific_piece(to + 8_u8, !self.side_to_move, PieceType::Pawn);
                }
                opt_captured_piece = Some(PieceType::Pawn);
                self.put_piece(to, self.side_to_move, PieceType::Pawn);
            }
            _ => {
                panic!("unable to make_move: invalid flags: {}", flags);
            }
        }

        // set castle rights
        let wk = self.pieces[PieceType::King as usize][Color::White as usize];
        let wr = self.pieces[PieceType::Rook as usize][Color::White as usize];
        let bk = self.pieces[PieceType::King as usize][Color::Black as usize];
        let br = self.pieces[PieceType::Rook as usize][Color::Black as usize];
        let castling_right_wk = ((wk & CASTLING_WK_K_POS_MASK) > 0 && (wr & CASTLING_WK_R_POS_MASK) > 0) as u8;
        let castling_right_wq = (((wk & CASTLING_WQ_K_POS_MASK) > 0 && (wr & CASTLING_WQ_R_POS_MASK) > 0) as u8) << 1;
        let castling_right_bk = (((bk & CASTLING_BK_K_POS_MASK) > 0 && (br & CASTLING_BK_R_POS_MASK) > 0) as u8) << 2;
        let castling_right_bq = (((bk & CASTLING_BQ_K_POS_MASK) > 0 && (br & CASTLING_BQ_R_POS_MASK) > 0) as u8) << 3;
        let new_castling_rights =
            castling_right_wk | castling_right_wq | castling_right_bk | castling_right_bq;
        self.castling_rights = self.castling_rights & new_castling_rights; // & operator makes sure castling rights cant be gained back

        // set new en passant target
        self.en_passant_target = opt_en_passant_target;

        // increase halfmove clock by 1 if no pawn was pushed and now piece captured
        let new_friendly_pawns = self.pieces[PieceType::Pawn as usize][self.side_to_move as usize];
        let new_total_pieces = self.all_occupied.count_ones();
        let pawns_changed = old_friendly_pawns ^ new_friendly_pawns;
        let piece_captured = (old_total_pieces - new_total_pieces) as u64;
        let increase_halfmove_clock = ((pawns_changed + piece_captured) == 0) as u8;
        self.halfmove_clock = increase_halfmove_clock * (self.halfmove_clock + 1);

        // increase full move number by 1 when black made a move
        self.fullmove_number += self.side_to_move as u16;

        // flip the side to move
        self.side_to_move = !self.side_to_move;

        UndoMove::new(
            mv,
            opt_captured_piece,
            old_en_passant_target,
            old_castling_rights,
            old_halfmove_clock,
        )
    }
}