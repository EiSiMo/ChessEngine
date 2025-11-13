use crate::r#move::*;
use crate::square::Square;
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
    fn rm_piece(
        &mut self,
        target_square: Square,
        color: Color,
    ) -> PieceType {
        let target_square_bitboard = target_square.to_bitboard();
        let piece_type = self.pieces_on_squares[target_square as usize].unwrap();

        self.pieces[piece_type as usize][color as usize] ^= target_square_bitboard;
        self.pieces_on_squares[target_square as usize] = None;

        // update occupancy helper bitboards
        self.occupied[color as usize] ^= target_square_bitboard;
        self.all_occupied ^= target_square_bitboard;
        self.empty_squares |= target_square_bitboard;

        piece_type
    }

    fn put_piece(&mut self, target_square: Square, color: Color, piece_type: PieceType) {
        let target_square_bitboard = target_square.to_bitboard();
        self.pieces[piece_type as usize][color as usize] |= target_square_bitboard;
        self.pieces_on_squares[target_square as usize] = Some(piece_type);

        // update occupancy helper bitboards
        self.occupied[color as usize] |= target_square_bitboard;
        self.all_occupied |= target_square_bitboard;
        self.empty_squares ^= target_square_bitboard;
    }

    fn move_piece(&mut self, from: Square, to: Square, color: Color) {
        let piece_type = self.rm_piece(from, color);
        self.put_piece(to, color, piece_type);
    }

    pub fn make_move(&mut self, mv: Move) -> UndoMove {
        // 1. Extract parts from move
        let from = mv.get_from();
        let to = mv.get_to();
        let flags = mv.get_flags();

        // 2. Save old state for UndoMove object
        let old_en_passant_target: Option<Square> = self.en_passant_target;
        let old_castling_rights: u8 = self.castling_rights;
        let old_halfmove_clock: u8 = self.halfmove_clock;

        // 3. Save pawns and total pieces for half move tracking
        let old_friendly_pawns = self.pieces[PieceType::Pawn as usize][self.side_to_move as usize];
        let old_total_pieces = self.all_occupied.count_ones();

        let mut opt_captured_piece: Option<PieceType> = None;
        let mut opt_en_passant_target: Option<Square> = None;

        // 4. Make the actual moves on the bitboard based on flag type
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
            _ => {
                panic!("unable to make_move: invalid flags: {}", flags);
            }
        }

        // 5. Update the castling rights
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
        self.castling_rights = self.castling_rights & new_castling_rights; // & operator makes sure castling rights can not be gained back

        // 6. Update the en passant target square
        self.en_passant_target = opt_en_passant_target;

        // 7. Update the halfmove clock
        let new_friendly_pawns = self.pieces[PieceType::Pawn as usize][self.side_to_move as usize];
        let new_total_pieces = self.all_occupied.count_ones();
        let pawns_changed = old_friendly_pawns ^ new_friendly_pawns;
        let piece_captured = (old_total_pieces - new_total_pieces) as u64;
        let increase_halfmove_clock = ((pawns_changed + piece_captured) == 0) as u8;
        self.halfmove_clock = increase_halfmove_clock * (self.halfmove_clock + 1);

        // 8. Increase the fullmove clock
        self.fullmove_number += self.side_to_move as u16;

        // 9. Flip the side to move
        self.side_to_move = !self.side_to_move;

        // 10. Create and return UndoMove object
        UndoMove::new(
            mv,
            opt_captured_piece,
            old_en_passant_target,
            old_castling_rights,
            old_halfmove_clock,
        )
    }
}