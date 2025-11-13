use std::fmt;
use crate::board::{Board, Color, PieceType};
use crate::r#move::{Move, MoveList, MOVE_FLAG_BK_CASTLE, MOVE_FLAG_BQ_CASTLE, MOVE_FLAG_PROMO_B, MOVE_FLAG_PROMO_B_CAP, MOVE_FLAG_PROMO_N, MOVE_FLAG_PROMO_N_CAP, MOVE_FLAG_PROMO_Q, MOVE_FLAG_PROMO_Q_CAP, MOVE_FLAG_PROMO_R, MOVE_FLAG_PROMO_R_CAP, MOVE_FLAG_WK_CASTLE, MOVE_FLAG_WQ_CASTLE, MOVE_FROM_MASK, MOVE_TO_MASK};

impl Board {
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
    pub fn print_bitboard(&self, name: &str, bitboard: u64) {
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

    /// Helper function to find which piece (as a char) is on a given square mask.
    pub fn get_piece_at(&self, sq_mask: u64) -> Option<char> {
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
}

impl fmt::Display for MoveList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.iter().map(|mv| mv.to_algebraic()).collect::<Vec<String>>().join(" "))
    }
}