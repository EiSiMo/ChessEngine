use std::slice;
use std::ops::{Index, IndexMut};
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
pub struct Move(pub(crate) u16);

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
        SQUARES[((self.0 & MOVE_TO_MASK) >> 6) as usize]
    }
}

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

    pub fn push(&mut self, mv: Move) {
        debug_assert!(self.count < 256, "Move list overflow!");

        self.moves[self.count] = mv;
        self.count += 1;
    }

    pub fn pull(&mut self) -> Option<Move> {
        if self.count > 0 {
            self.count -= 1;
            return Some(self.moves[self.count]);
        }
        None
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.moves[..self.count].swap(a, b);
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn iter(&self) -> slice::Iter<'_, Move> {
        self.moves[..self.count].iter()
    }

    pub fn contains(&self, mv: &Move) -> bool {
        self.moves.contains(mv)
    }

    pub fn clear(&mut self) { self.count = 0 }
}

impl Index<usize> for MoveList {
    type Output = Move;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.moves[..self.count][index]
    }
}

impl IndexMut<usize> for MoveList {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.moves[..self.count][index]
    }
}

pub struct UndoMove {
    pub mv: Move,
    pub captured_piece: Option<PieceType>,
    pub old_en_passant_square: Option<Square>,
    pub old_castling_rights: u8,
    pub old_halfmove_clock: u8,
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