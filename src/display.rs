use crate::board::{Board, Color, PieceType};
use crate::r#move::MoveList;
use crate::square::Square;
use std::fmt;

impl Board {
    /// Prints the board as a human-readable ASCII grid.
    pub fn pretty_print_ascii(&self) {
        println!("\n   a b c d e f g h");
        for rank in (0..=7).rev() {
            print!("{}  ", rank + 1); // Rank annotation
            for file in 0..=7 {
                let sq = (rank * 8 + file) as u8;
                let mask = 1u64 << sq;

                if let Some(piece) = self.get_piece_at_unicode(mask) {
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

    /// Helper function to find which piece (as a FEN char) is on a given square mask.
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

    /// Helper function to find which piece (as a unicode char) is on a given square mask.
    pub fn get_piece_at_unicode(&self, sq_mask: u64) -> Option<char> {
        let white = Color::White as usize;
        let black = Color::Black as usize;

        if (self.pieces[PieceType::Pawn as usize][white] & sq_mask) != 0 {
            return Some('♙');
        }
        if (self.pieces[PieceType::Pawn as usize][black] & sq_mask) != 0 {
            return Some('♟');
        }
        if (self.pieces[PieceType::Knight as usize][white] & sq_mask) != 0 {
            return Some('♘');
        }
        if (self.pieces[PieceType::Knight as usize][black] & sq_mask) != 0 {
            return Some('♞');
        }
        if (self.pieces[PieceType::Bishop as usize][white] & sq_mask) != 0 {
            return Some('♗');
        }
        if (self.pieces[PieceType::Bishop as usize][black] & sq_mask) != 0 {
            return Some('♝');
        }
        if (self.pieces[PieceType::Rook as usize][white] & sq_mask) != 0 {
            return Some('♖');
        }
        if (self.pieces[PieceType::Rook as usize][black] & sq_mask) != 0 {
            return Some('♜');
        }
        if (self.pieces[PieceType::Queen as usize][white] & sq_mask) != 0 {
            return Some('♕');
        }
        if (self.pieces[PieceType::Queen as usize][black] & sq_mask) != 0 {
            return Some('♛');
        }
        if (self.pieces[PieceType::King as usize][white] & sq_mask) != 0 {
            return Some('♔');
        }
        if (self.pieces[PieceType::King as usize][black] & sq_mask) != 0 {
            return Some('♚');
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
        write!(
            f,
            "{}",
            &self
                .iter()
                .map(|mv| mv.to_algebraic())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Square::A1 => write!(f, "A1"),
            Square::B1 => write!(f, "B1"),
            Square::C1 => write!(f, "C1"),
            Square::D1 => write!(f, "D1"),
            Square::E1 => write!(f, "E1"),
            Square::F1 => write!(f, "F1"),
            Square::G1 => write!(f, "G1"),
            Square::H1 => write!(f, "H1"),
            Square::A2 => write!(f, "A2"),
            Square::B2 => write!(f, "B2"),
            Square::C2 => write!(f, "C2"),
            Square::D2 => write!(f, "D2"),
            Square::E2 => write!(f, "E2"),
            Square::F2 => write!(f, "F2"),
            Square::G2 => write!(f, "G2"),
            Square::H2 => write!(f, "H2"),
            Square::A3 => write!(f, "A3"),
            Square::B3 => write!(f, "B3"),
            Square::C3 => write!(f, "C3"),
            Square::D3 => write!(f, "D3"),
            Square::E3 => write!(f, "E3"),
            Square::F3 => write!(f, "F3"),
            Square::G3 => write!(f, "G3"),
            Square::H3 => write!(f, "H3"),
            Square::A4 => write!(f, "A4"),
            Square::B4 => write!(f, "B4"),
            Square::C4 => write!(f, "C4"),
            Square::D4 => write!(f, "D4"),
            Square::E4 => write!(f, "E4"),
            Square::F4 => write!(f, "F4"),
            Square::G4 => write!(f, "G4"),
            Square::H4 => write!(f, "H4"),
            Square::A5 => write!(f, "A5"),
            Square::B5 => write!(f, "B5"),
            Square::C5 => write!(f, "C5"),
            Square::D5 => write!(f, "D5"),
            Square::E5 => write!(f, "E5"),
            Square::F5 => write!(f, "F5"),
            Square::G5 => write!(f, "G5"),
            Square::H5 => write!(f, "H5"),
            Square::A6 => write!(f, "A6"),
            Square::B6 => write!(f, "B6"),
            Square::C6 => write!(f, "C6"),
            Square::D6 => write!(f, "D6"),
            Square::E6 => write!(f, "E6"),
            Square::F6 => write!(f, "F6"),
            Square::G6 => write!(f, "G6"),
            Square::H6 => write!(f, "H6"),
            Square::A7 => write!(f, "A7"),
            Square::B7 => write!(f, "B7"),
            Square::C7 => write!(f, "C7"),
            Square::D7 => write!(f, "D7"),
            Square::E7 => write!(f, "E7"),
            Square::F7 => write!(f, "F7"),
            Square::G7 => write!(f, "G7"),
            Square::H7 => write!(f, "H7"),
            Square::A8 => write!(f, "A8"),
            Square::B8 => write!(f, "B8"),
            Square::C8 => write!(f, "C8"),
            Square::D8 => write!(f, "D8"),
            Square::E8 => write!(f, "E8"),
            Square::F8 => write!(f, "F8"),
            Square::G8 => write!(f, "G8"),
            Square::H8 => write!(f, "H8")
        }
    }
}
