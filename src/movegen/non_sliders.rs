use crate::board::*;
use crate::r#move::*;
use crate::square::*;
use super::tables::{KING_ATTACKS, KNIGHT_ATTACKS};

pub fn generate_knight_moves(board: &Board, list: &mut MoveList) {
    let enemy_occupied = board.occupied[!board.side_to_move as usize];
    let mut friendly_knights = board.pieces[PieceType::Knight as usize][board.side_to_move as usize];

    while friendly_knights != 0 {
        let square = SQUARES[friendly_knights.trailing_zeros() as usize];
        let mut attacks = KNIGHT_ATTACKS[square as usize] & !board.occupied[board.side_to_move as usize];

        while attacks != 0 {
            let attack = SQUARES[attacks.trailing_zeros() as usize];
            let attack_bb = 1u64 << attack as u64;

            let flags = if (enemy_occupied & attack_bb) != 0 {
                MOVE_FLAG_CAPTURE
            } else {
                MOVE_FLAG_QUIET
            };

            let mv = Move::new(square, attack, flags);
            list.push(mv);
            attacks &= attacks - 1;
        }
        friendly_knights &= friendly_knights - 1;
    }
}

pub fn generate_king_moves(board: &Board, list: &mut MoveList) {
    let enemy_occupied = board.occupied[!board.side_to_move as usize];
    let friendly_king = board.pieces[PieceType::King as usize][board.side_to_move as usize];

    if friendly_king == 0 {
        return;
    }

    let square = SQUARES[friendly_king.trailing_zeros() as usize];

    // 1. Generate standard king moves
    let mut attacks = KING_ATTACKS[square as usize] & !board.occupied[board.side_to_move as usize];
    while attacks != 0 {
        let attack = SQUARES[attacks.trailing_zeros() as usize];
        let attack_bb = 1u64 << attack as u64;

        let flags = if (enemy_occupied & attack_bb) != 0 {
            MOVE_FLAG_CAPTURE
        } else {
            MOVE_FLAG_QUIET
        };

        let mv = Move::new(square, attack, flags);
        list.push(mv);
        attacks &= attacks - 1;
    }

    // 2. Generate castling king moves
    if board.side_to_move == Color::White {
        // Kingside (OO)
        if (board.castling_rights & CASTLING_WK) != 0 {
            if (board.all_occupied & CASTLING_WK_MASK) == 0 {
                list.push(Move::new(Square::E1, Square::G1, MOVE_FLAG_WK_CASTLE));
            }
        }
        // Queenside (OOO)
        if (board.castling_rights & CASTLING_WQ) != 0 {
            if (board.all_occupied & CASTLING_WQ_MASK) == 0 {
                list.push(Move::new(Square::E1, Square::C1, MOVE_FLAG_WQ_CASTLE));
            }
        }
    } else { // Black
        // Kingside (OO)
        if (board.castling_rights & CASTLING_BK) != 0 {
            if (board.all_occupied & CASTLING_BK_MASK) == 0 {
                list.push(Move::new(Square::E8, Square::G8, MOVE_FLAG_BK_CASTLE));
            }
        }
        // Queenside (OOO)
        if (board.castling_rights & CASTLING_BQ) != 0 {
            if (board.all_occupied & CASTLING_BQ_MASK) == 0 {
                list.push(Move::new(Square::E8, Square::C8, MOVE_FLAG_BQ_CASTLE));
            }
        }
    }
}
