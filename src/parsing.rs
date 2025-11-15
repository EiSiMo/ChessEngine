use std::mem;
use crate::board::*;
use crate::r#move::*;
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
                            pieces_on_squares[sq as usize] = Some(PieceType::Pawn);
                        }
                        'N' => {
                            pieces[PieceType::Knight as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Knight);
                        }
                        'B' => {
                            pieces[PieceType::Bishop as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Bishop);
                        }
                        'R' => {
                            pieces[PieceType::Rook as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Rook);
                        }
                        'Q' => {
                            pieces[PieceType::Queen as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::Queen);
                        }
                        'K' => {
                            pieces[PieceType::King as usize][color_idx] |= mask;
                            pieces_on_squares[sq as usize] = Some(PieceType::King);
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

impl Move {
    /// Converts a square index (0-63) to algebraic notation (e.g., 0 -> "a1", 63 -> "h8").
    fn square_val_to_alg(val: u16) -> String {
        let file = (b'a' + (val % 8) as u8) as char;
        let rank = (b'1' + (val / 8) as u8) as char;
        format!("{}{}", file, rank)
    }

    /// Converts algebraic notation (e.g., "a1") to a Square.
    /// Assumes valid input.
    fn alg_to_square(alg: &str) -> Square {
        let file = (alg.as_bytes()[0] - b'a') as u8;
        let rank = (alg.as_bytes()[1] - b'1') as u8;
        let sq_index = rank * 8 + file;
        // This is unsafe, but we assume valid algebraic notation
        unsafe { mem::transmute::<u8, Square>(sq_index) }
    }

    /// Converts the move to coordinate notation (e.g., "e2e4", "e7e8q", "e1g1").
    pub fn to_algebraic(&self) -> String {
        let flags = self.get_flags();

        let from_val = self.0 & MOVE_FROM_MASK;
        let to_val = (self.0 & MOVE_TO_MASK) >> 6;

        let from_str = Self::square_val_to_alg(from_val);
        let to_str = Self::square_val_to_alg(to_val);

        // Check if it's any promotion type (1xxx)
        if (flags & 0b1000_0000_0000_0000) != 0 {
            let promo_char = match flags {
                MOVE_FLAG_PROMO_N | MOVE_FLAG_PROMO_N_CAP => 'n',
                MOVE_FLAG_PROMO_B | MOVE_FLAG_PROMO_B_CAP => 'b',
                MOVE_FLAG_PROMO_R | MOVE_FLAG_PROMO_R_CAP => 'r',
                MOVE_FLAG_PROMO_Q | MOVE_FLAG_PROMO_Q_CAP => 'q',
                _ => '?', // Should not happen
            };
            format!("{}{}{}", from_str, to_str, promo_char)
        } else {
            // This covers Quiet, DoublePawn, Capture, EnPassant, Castles
            format!("{}{}", from_str, to_str)
        }
    }

    /// Creates a Move from algebraic notation (e.g., "e2e4") and a board state.
    /// Assumes the move is valid and legal for the given board state.
    pub fn from_algebraic(s: &str, board: &Board) -> Move {
        let from_sq = Self::alg_to_square(&s[0..2]);
        let to_sq = Self::alg_to_square(&s[2..4]);

        let moving_piece = board.pieces_on_squares[from_sq as usize]
            .expect("Invalid move: No piece on 'from' square.");

        let is_capture = board.pieces_on_squares[to_sq as usize].is_some();

        // 1. Handle Promotions
        if s.len() == 5 {
            let promo_char = s.chars().nth(4).unwrap();
            match (promo_char, is_capture) {
                ('q', false) => return Move::new(from_sq, to_sq, MOVE_FLAG_PROMO_Q),
                ('n', false) => return Move::new(from_sq, to_sq, MOVE_FLAG_PROMO_N),
                ('r', false) => return Move::new(from_sq, to_sq, MOVE_FLAG_PROMO_R),
                ('b', false) => return Move::new(from_sq, to_sq, MOVE_FLAG_PROMO_B),
                ('q', true)  => return Move::new(from_sq, to_sq, MOVE_FLAG_PROMO_Q_CAP),
                ('n', true)  => return Move::new(from_sq, to_sq, MOVE_FLAG_PROMO_N_CAP),
                ('r', true)  => return Move::new(from_sq, to_sq, MOVE_FLAG_PROMO_R_CAP),
                ('b', true)  => return Move::new(from_sq, to_sq, MOVE_FLAG_PROMO_B_CAP),
                _ => panic!("Invalid promotion character"),
            }
        }

        // 2. Handle Castling
        if moving_piece == PieceType::King {
            let from_idx = from_sq as u8;
            let to_idx = to_sq as u8;

            // White King Side: e1g1 (idx 4 -> 6)
            if from_idx == 4 && to_idx == 6 { return Move::new(from_sq, to_sq, MOVE_FLAG_WK_CASTLE); }
            // White Queen Side: e1c1 (idx 4 -> 2)
            if from_idx == 4 && to_idx == 2 { return Move::new(from_sq, to_sq, MOVE_FLAG_WQ_CASTLE); }
            // Black King Side: e8g8 (idx 60 -> 62)
            if from_idx == 60 && to_idx == 62 { return Move::new(from_sq, to_sq, MOVE_FLAG_BK_CASTLE); }
            // Black Queen Side: e8c8 (idx 60 -> 58)
            if from_idx == 60 && to_idx == 58 { return Move::new(from_sq, to_sq, MOVE_FLAG_BQ_CASTLE); }
        }

        // 3. Handle Pawn Special Moves
        if moving_piece == PieceType::Pawn {
            // Double Pawn Push
            let rank_diff = (to_sq as i8 - from_sq as i8).abs();
            if rank_diff == 16 {
                return Move::new(from_sq, to_sq, MOVE_FLAG_DOUBLE_PAWN);
            }

            // En Passant
            // Must be diagonal move, to the en_passant_target square, and not a normal capture
            if Some(to_sq) == board.en_passant_target && !is_capture {
                let from_file = from_sq as u8 % 8;
                let to_file = to_sq as u8 % 8;
                if from_file != to_file {
                    return Move::new(from_sq, to_sq, MOVE_FLAG_EN_PASSANT);
                }
            }
        }

        // 4. Handle Normal Captures / Quiet Moves
        if is_capture {
            Move::new(from_sq, to_sq, MOVE_FLAG_CAPTURE)
        } else {
            Move::new(from_sq, to_sq, MOVE_FLAG_QUIET)
        }
    }
}