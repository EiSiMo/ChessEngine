use crate::r#move::*;
use crate::square::Square;
use crate::zobrist::{self, zobrist_keys};
use std::ops::Not;

pub const CASTLING_WK_FLAG: u8 = 1;
pub const CASTLING_WK_MASK: u64 = 96;
pub const CASTLING_WK_K_POS_MASK: u64 = 16;
pub const CASTLING_WK_R_POS_MASK: u64 = 128;

pub const CASTLING_WQ_FLAG: u8 = 2;
pub const CASTLING_WQ_MASK: u64 = 14;
pub const CASTLING_WQ_K_POS_MASK: u64 = 16;
pub const CASTLING_WQ_R_POS_MASK: u64 = 1;

pub const CASTLING_BK_FLAG: u8 = 4;
pub const CASTLING_BK_MASK: u64 = 6917529027641081856;
pub const CASTLING_BK_K_POS_MASK: u64 = 1152921504606846976;
pub const CASTLING_BK_R_POS_MASK: u64 = 9223372036854775808;

pub const CASTLING_BQ_FLAG: u8 = 8;
pub const CASTLING_BQ_MASK: u64 = 1008806316530991104;
pub const CASTLING_BQ_K_POS_MASK: u64 = 1152921504606846976;
pub const CASTLING_BQ_R_POS_MASK: u64 = 72057594037927936;

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
    pub pieces_on_squares: [Option<PieceType>; 64],

    pub occupied: [u64; 2],
    pub all_occupied: u64,
    pub empty_squares: u64,

    pub castling_rights: u8,
    pub en_passant_target: Option<Square>,

    pub halfmove_clock: u8,
    pub fullmove_number: u16,

    pub hash: u64,
}

impl Board {
    fn ep_file_index(ep: Option<Square>) -> usize {
        match ep {
            Some(sq) => (sq as usize) % 8,
            None => 8,
        }
    }

    pub fn recalculate_hash(&mut self) {
        let keys = zobrist_keys();
        let mut hash = 0;

        for sq in 0..64 {
            if let Some(pt) = self.pieces_on_squares[sq] {
                let color = if (self.pieces[pt as usize][Color::White as usize] & (1 << sq)) != 0 {
                    Color::White
                } else {
                    Color::Black
                };
                hash ^= keys.pieces[zobrist::piece_index(pt, color)][sq];
            }
        }
        hash ^= keys.castling[self.castling_rights as usize];
        hash ^= keys.en_passant[Self::ep_file_index(self.en_passant_target)];

        if self.side_to_move == Color::Black {
            hash ^= keys.side_to_move;
        }

        self.hash = hash;
    }
    
    fn rm_piece(
        &mut self,
        target_square: Square,
        color: Color,
    ) -> PieceType {
        let target_square_bitboard = target_square.to_bitboard();
        let piece_type = self.pieces_on_squares[target_square as usize].unwrap();

        let keys = zobrist_keys();
        self.hash ^= keys.pieces[zobrist::piece_index(piece_type, color)][target_square as usize];

        self.pieces[piece_type as usize][color as usize] ^= target_square_bitboard;
        self.pieces_on_squares[target_square as usize] = None;

        self.occupied[color as usize] ^= target_square_bitboard;
        self.all_occupied ^= target_square_bitboard;
        self.empty_squares |= target_square_bitboard;

        piece_type
    }

    fn put_piece(&mut self, target_square: Square, color: Color, piece_type: PieceType) {
        let target_square_bitboard = target_square.to_bitboard();

        let keys = zobrist_keys();
        self.hash ^= keys.pieces[zobrist::piece_index(piece_type, color)][target_square as usize];

        self.pieces[piece_type as usize][color as usize] |= target_square_bitboard;
        self.pieces_on_squares[target_square as usize] = Some(piece_type);

        self.occupied[color as usize] |= target_square_bitboard;
        self.all_occupied |= target_square_bitboard;
        self.empty_squares ^= target_square_bitboard;
    }

    fn move_piece(&mut self, from: Square, to: Square, color: Color) {
        let piece_type = self.rm_piece(from, color);
        self.put_piece(to, color, piece_type);
    }

    pub fn make_move(&mut self, mv: Move) -> UndoMove {
        let keys = zobrist_keys();

        self.hash ^= keys.en_passant[Self::ep_file_index(self.en_passant_target)];
        self.hash ^= keys.castling[self.castling_rights as usize];

        let from = mv.get_from();
        let to = mv.get_to();
        let flags = mv.get_flags();

        let old_en_passant_target: Option<Square> = self.en_passant_target;
        let old_castling_rights: u8 = self.castling_rights;
        let old_halfmove_clock: u8 = self.halfmove_clock;

        let old_friendly_pawns = self.pieces[PieceType::Pawn as usize][self.side_to_move as usize];
        let old_total_pieces = self.all_occupied.count_ones();

        let mut opt_captured_piece: Option<PieceType> = None;
        let mut opt_en_passant_target: Option<Square> = None;

        match flags {
            MOVE_FLAG_QUIET => {
                self.move_piece(from, to, self.side_to_move);
            }
            MOVE_FLAG_CAPTURE => {
                opt_captured_piece = Some(self.rm_piece(to, !self.side_to_move));
                self.move_piece(from, to, self.side_to_move);
            }
            MOVE_FLAG_DOUBLE_PAWN => {
                self.move_piece(from, to, self.side_to_move);
                opt_en_passant_target = Some(to + (self.side_to_move as i8 * 16 - 8));
            }
            MOVE_FLAG_PROMO_Q => {
                self.rm_piece(from, self.side_to_move);
                self.put_piece(to, self.side_to_move, PieceType::Queen);
            }
            MOVE_FLAG_PROMO_N => {
                self.rm_piece(from, self.side_to_move);
                self.put_piece(to, self.side_to_move, PieceType::Knight);
            }
            MOVE_FLAG_PROMO_B => {
                self.rm_piece(from, self.side_to_move);
                self.put_piece(to, self.side_to_move, PieceType::Bishop);
            }
            MOVE_FLAG_PROMO_R => {
                self.rm_piece(from, self.side_to_move);
                self.put_piece(to, self.side_to_move, PieceType::Rook);
            }
            MOVE_FLAG_PROMO_Q_CAP => {
                self.rm_piece(from, self.side_to_move);
                opt_captured_piece = Some(self.rm_piece(to, !self.side_to_move));
                self.put_piece(to, self.side_to_move, PieceType::Queen);
            }
            MOVE_FLAG_PROMO_N_CAP => {
                self.rm_piece(from, self.side_to_move);
                opt_captured_piece = Some(self.rm_piece(to, !self.side_to_move));
                self.put_piece(to, self.side_to_move, PieceType::Knight);
            }
            MOVE_FLAG_PROMO_B_CAP => {
                self.rm_piece(from, self.side_to_move);
                opt_captured_piece = Some(self.rm_piece(to, !self.side_to_move));
                self.put_piece(to, self.side_to_move, PieceType::Bishop);
            }
            MOVE_FLAG_PROMO_R_CAP => {
                self.rm_piece(from, self.side_to_move);
                opt_captured_piece = Some(self.rm_piece(to, !self.side_to_move));
                self.put_piece(to, self.side_to_move, PieceType::Rook);
            }
            MOVE_FLAG_WK_CASTLE => {
                self.move_piece(Square::E1, Square::G1, self.side_to_move);
                self.move_piece(Square::H1, Square::F1, self.side_to_move);
            }
            MOVE_FLAG_BK_CASTLE => {
                self.move_piece(Square::E8, Square::G8, self.side_to_move);
                self.move_piece(Square::H8, Square::F8, self.side_to_move);
            }
            MOVE_FLAG_WQ_CASTLE => {
                self.move_piece(Square::E1, Square::C1, self.side_to_move);
                self.move_piece(Square::A1, Square::D1, self.side_to_move);
            }
            MOVE_FLAG_BQ_CASTLE => {
                self.move_piece(Square::E8, Square::C8, self.side_to_move);
                self.move_piece(Square::A8, Square::D8, self.side_to_move);
            }
            MOVE_FLAG_EN_PASSANT => {
                self.move_piece(from, to, self.side_to_move);
                self.rm_piece(to + (self.side_to_move as i8 * 16 - 8), !self.side_to_move);
                opt_captured_piece = Some(PieceType::Pawn);
            }
            _ => { panic!("unable to make_move: invalid flags: {}", flags); }
        }

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
        self.castling_rights = self.castling_rights & new_castling_rights;

        self.en_passant_target = opt_en_passant_target;

        self.hash ^= keys.en_passant[Self::ep_file_index(self.en_passant_target)];
        self.hash ^= keys.castling[self.castling_rights as usize];
        self.hash ^= keys.side_to_move;

        let new_friendly_pawns = self.pieces[PieceType::Pawn as usize][self.side_to_move as usize];
        let new_total_pieces = self.all_occupied.count_ones();
        let pawns_changed = old_friendly_pawns ^ new_friendly_pawns;
        let piece_captured = (old_total_pieces - new_total_pieces) as u64;
        let increase_halfmove_clock = ((pawns_changed + piece_captured) == 0) as u8;
        self.halfmove_clock = increase_halfmove_clock * (self.halfmove_clock + 1);

        self.fullmove_number += self.side_to_move as u16;

        self.side_to_move = !self.side_to_move;

        UndoMove::new(
            mv,
            opt_captured_piece,
            old_en_passant_target,
            old_castling_rights,
            old_halfmove_clock,
        )
    }

    pub fn undo_move(&mut self, undo_info: UndoMove) {
        let keys = zobrist_keys();

        self.hash ^= keys.side_to_move;
        self.hash ^= keys.castling[self.castling_rights as usize];
        self.hash ^= keys.en_passant[Self::ep_file_index(self.en_passant_target)];

        self.castling_rights = undo_info.old_castling_rights;
        self.en_passant_target = undo_info.old_en_passant_square;
        self.halfmove_clock = undo_info.old_halfmove_clock;

        self.hash ^= keys.castling[self.castling_rights as usize];
        self.hash ^= keys.en_passant[Self::ep_file_index(self.en_passant_target)];

        self.side_to_move = !self.side_to_move;

        self.fullmove_number -= self.side_to_move as u16;

        let mv = undo_info.mv;
        let from = mv.get_from();
        let to = mv.get_to();
        let flags = mv.get_flags();

        match flags {
            MOVE_FLAG_QUIET => {
                self.move_piece(to, from, self.side_to_move);
            }
            MOVE_FLAG_CAPTURE => {
                self.move_piece(to, from, self.side_to_move);
                self.put_piece(to, !self.side_to_move, undo_info.captured_piece.unwrap());
            }
            MOVE_FLAG_DOUBLE_PAWN => {
                self.move_piece(to, from, self.side_to_move);
            }
            MOVE_FLAG_PROMO_Q | MOVE_FLAG_PROMO_N | MOVE_FLAG_PROMO_B | MOVE_FLAG_PROMO_R => {
                self.rm_piece(to, self.side_to_move);
                self.put_piece(from, self.side_to_move, PieceType::Pawn);
            }
            MOVE_FLAG_PROMO_Q_CAP | MOVE_FLAG_PROMO_N_CAP | MOVE_FLAG_PROMO_B_CAP | MOVE_FLAG_PROMO_R_CAP => {
                self.rm_piece(to, self.side_to_move);
                self.put_piece(from, self.side_to_move, PieceType::Pawn);
                self.put_piece(to, !self.side_to_move, undo_info.captured_piece.unwrap());
            }
            MOVE_FLAG_WK_CASTLE => {
                self.move_piece(Square::G1, Square::E1, self.side_to_move);
                self.move_piece(Square::F1, Square::H1, self.side_to_move);
            }
            MOVE_FLAG_BK_CASTLE => {
                self.move_piece(Square::G8, Square::E8, self.side_to_move);
                self.move_piece(Square::F8, Square::H8, self.side_to_move);
            }
            MOVE_FLAG_WQ_CASTLE => {
                self.move_piece(Square::C1, Square::E1, self.side_to_move);
                self.move_piece(Square::D1, Square::A1, self.side_to_move);
            }
            MOVE_FLAG_BQ_CASTLE => {
                self.move_piece(Square::C8, Square::E8, self.side_to_move);
                self.move_piece(Square::D8, Square::A8, self.side_to_move);
            }
            MOVE_FLAG_EN_PASSANT => {
                self.move_piece(to, from, self.side_to_move);
                let captured_pawn_square = to + (self.side_to_move as i8 * 16 - 8);
                self.put_piece(captured_pawn_square, !self.side_to_move, PieceType::Pawn);
            }
            _ => { panic!("unable to unmake_move: invalid flags: {}", flags); }
        }
    }
}