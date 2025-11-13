use std::slice;
use std::fmt;
use crate::square::Square;
use crate::board::PieceType;
use crate::square::SQUARES;

// BIT 0 - 5: FROM SQUARE (0-63)
pub const MOVE_FROM_MASK: u16 = 0b0000_0000_0011_1111;

// BIT 6 - 11: TO SQUARE (0-63)
pub const MOVE_TO_MASK: u16 = 0b0000_1111_1100_0000;

// BIT 12 - 15: FLAGS (4 bits)
pub const MOVE_FLAG_MASK: u16 = 0b1111_0000_0000_0000;

pub const MOVE_FLAG_QUIET: u16 = 0b0000_0000_0000_0000;
pub const MOVE_FLAG_CAPTURE: u16 = 0b0001_0000_0000_0000;
pub const MOVE_FLAG_DOUBLE_PAWN: u16 = 0b0010_0000_0000_0000;
pub const MOVE_FLAG_EN_PASSANT: u16 = 0b0011_0000_0000_0000;

// Castle flags
pub const MOVE_MASK_CASTLE: u16 = 0b1100_0000_0000_0000;
pub const MOVE_FLAG_CASTLE_TRUE: u16 = 0b0100_0000_0000_0000;

pub const MOVE_FLAG_WK_CASTLE: u16 = 0b0100_0000_0000_0000;
pub const MOVE_FLAG_WQ_CASTLE: u16 = 0b0101_0000_0000_0000;
pub const MOVE_FLAG_BK_CASTLE: u16 = 0b0110_0000_0000_0000;
pub const MOVE_FLAG_BQ_CASTLE: u16 = 0b0111_0000_0000_0000;

// Promotion flags (use the 1xxx bits)
// We combine capture flag with promotion type
pub const MOVE_MASK_PROMO: u16 = 0b1000_0000_0000_0000;
pub const MOVE_FLAG_PROMO: u16 = 0b1000_0000_0000_0000;

pub const MOVE_FLAG_PROMO_N: u16 = 0b1000_0000_0000_0000;
pub const MOVE_FLAG_PROMO_B: u16 = 0b1010_0000_0000_0000;
pub const MOVE_FLAG_PROMO_R: u16 = 0b1100_0000_0000_0000;
pub const MOVE_FLAG_PROMO_Q: u16 = 0b1110_0000_0000_0000;
pub const MOVE_FLAG_PROMO_N_CAP: u16 = 0b1001_0000_0000_0000;
pub const MOVE_FLAG_PROMO_B_CAP: u16 = 0b1011_0000_0000_0000;
pub const MOVE_FLAG_PROMO_R_CAP: u16 = 0b1101_0000_0000_0000;
pub const MOVE_FLAG_PROMO_Q_CAP: u16 = 0b1111_0000_0000_0000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move(u16);

impl Move {
    pub fn new(from: Square, to: Square, flags: u16) -> Move {
        Move(flags |
            ((to as u16) << 6 ) |
            from as u16)
    }


    #[inline(always)]
    pub fn get_flags(&self) -> u16 {
        self.0 & MOVE_FLAG_MASK
    }

    #[inline(always)]
    pub fn get_from(&self) -> Square {
        SQUARES[(self.0 & MOVE_FROM_MASK) as usize]
    }

    #[inline(always)]
    pub fn get_to(&self) -> Square {
        // --- KORREKTUR HIER ---
        // Die Klammern um (self.0 & MOVE_TO_MASK) sind entscheidend
        SQUARES[((self.0 & MOVE_TO_MASK) >> 6) as usize]
    }


    /// Converts a square index (0-63) to algebraic notation (e.g., 0 -> "a1", 63 -> "h8").
    fn square_val_to_alg(val: u16) -> String {
        let file = (b'a' + (val % 8) as u8) as char;
        let rank = (b'1' + (val / 8) as u8) as char;
        format!("{}{}", file, rank)
    }

    /// Converts the move to coordinate notation (e.g., "e2e4", "e7e8q", "e1g1").
    pub fn to_algebraic(&self) -> String {
        let flags = self.get_flags();

        // Handle castling first. In this new format, the "to" square is
        // the *king's* destination square (g1/c1 or g8/c8).
        // Your old implementation reading the file is still fine.
        if (flags == MOVE_FLAG_WK_CASTLE) || (flags == MOVE_FLAG_BK_CASTLE) {
            return "O-O".to_string();
        }
        if (flags == MOVE_FLAG_WQ_CASTLE) || (flags == MOVE_FLAG_BQ_CASTLE) {
            return "O-O-O".to_string();
        }

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
            // This covers Quiet, DoublePawn, Capture, EnPassant
            format!("{}{}", from_str, to_str)
        }
    }
}

// ... Rest des MoveList-Codes bleibt exakt gleich ...
// (MoveList, new, push, len, is_empty, iter, impl fmt::Display)
pub struct MoveList {
    moves: [Move; 256],
    count: usize,
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            moves: [unsafe { std::mem::zeroed() }; 256],
            count: 0,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, mv: Move) {
        debug_assert!(self.count < 256, "Move list overflow!");

        self.moves[self.count] = mv;
        self.count += 1;
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.count
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    #[inline(always)]
    pub fn iter(&self) -> slice::Iter<'_, Move> {
        self.moves[..self.count].iter()
    }
}

impl fmt::Display for MoveList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.iter().map(|mv| mv.to_algebraic()).collect::<Vec<String>>().join(" "))
    }
}


pub struct UndoMove {
    mv: Move,
    captured_piece: Option<PieceType>,
    old_en_passant_square: Option<Square>,
    old_castling_rights: u8,
    old_halfmove_clock: u8,
}

impl UndoMove {
    pub fn new(mv: Move,
               captured_piece: Option<PieceType>,
               old_en_passant_square: Option<Square>,
               old_castling_rights: u8,
               old_halfmove_clock: u8) -> Self {
        Self {
            mv,
            captured_piece,
            old_en_passant_square,
            old_castling_rights,
            old_halfmove_clock
        }

    }
}