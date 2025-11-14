use crate::board::*;
use crate::movegen::legal_check::is_square_attacked;
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

    // TODO Optimize the is attacked testing on castling
    // 2. Generate castling king moves
    if board.side_to_move == Color::White {
        // King must not be in check to castle
        if is_square_attacked(board, Square::E1, Color::Black) {
            return;
        }

        // Kingside (OO)
        if (board.castling_rights & CASTLING_WK_FLAG) != 0 {
            if (board.all_occupied & CASTLING_WK_MASK) == 0 {
                // Check F1 (path) and G1 (landing)
                if !is_square_attacked(board, Square::F1, Color::Black) &&
                    !is_square_attacked(board, Square::G1, Color::Black) {
                    list.push(Move::new(Square::E1, Square::G1, MOVE_FLAG_WK_CASTLE));
                }
            }
        }
        // Queenside (OOO)
        if (board.castling_rights & CASTLING_WQ_FLAG) != 0 {
            if (board.all_occupied & CASTLING_WQ_MASK) == 0 {
                // Check D1 (path) and C1 (landing). B1 is irrelevant.
                if !is_square_attacked(board, Square::D1, Color::Black) &&
                    !is_square_attacked(board, Square::C1, Color::Black) {
                    list.push(Move::new(Square::E1, Square::C1, MOVE_FLAG_WQ_CASTLE));
                }
            }
        }
    } else { // Black
        // King must not be in check to castle
        if is_square_attacked(board, Square::E8, Color::White) {
            return;
        }

        // Kingside (OO)
        if (board.castling_rights & CASTLING_BK_FLAG) != 0 {
            if (board.all_occupied & CASTLING_BK_MASK) == 0 {
                // Check F8 (path) and G8 (landing)
                if !is_square_attacked(board, Square::F8, Color::White) &&
                    !is_square_attacked(board, Square::G8, Color::White) {
                    list.push(Move::new(Square::E8, Square::G8, MOVE_FLAG_BK_CASTLE));
                }
            }
        }
        // Queenside (OOO)
        if (board.castling_rights & CASTLING_BQ_FLAG) != 0 {
            if (board.all_occupied & CASTLING_BQ_MASK) == 0 {
                // Check D8 (path) and C8 (landing). B8 is irrelevant.
                if !is_square_attacked(board, Square::D8, Color::White) &&
                    !is_square_attacked(board, Square::C8, Color::White) {
                    list.push(Move::new(Square::E8, Square::C8, MOVE_FLAG_BQ_CASTLE));
                }
            }
        }
    }
}