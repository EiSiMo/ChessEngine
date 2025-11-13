use std::mem;
use crate::board::{Board, Color, PieceType, CASTLING_BK_FLAG, CASTLING_BQ_FLAG, CASTLING_WK_FLAG, CASTLING_WQ_FLAG};
use crate::square::Square;

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
}