use crate::board::*;
use crate::r#move::*;
use crate::square::*;

pub const RANK1_MASK: u64 = 255; // A1 - H1
pub const RANK2_MASK: u64 = 65280; // A2 - H2
pub const RANK3_MASK: u64 = 16711680; // A3 - H3
pub const RANK4_MASK: u64 = 4278190080; // A4 - H4
pub const RANK5_MASK: u64 = 1095216660480; // A5 - H5
pub const RANK6_MASK: u64 = 280375465082880; // A6 - H6
pub const RANK7_MASK: u64 = 71776119061217280; // A7 - H7
pub const RANK8_MASK: u64 = 18374686479671623680; // A8 - H8


pub const PAWN_A_SIDE_CAPTURE_MASK_WITHE: u64 = 280371153272574; // B1 - H6 (omitted promotions)
pub const PAWN_H_SIDE_CAPTURE_MASK_WITHE: u64 = 140185576636287; // A1- G6 (omitted promotions)

pub const PAWN_A_SIDE_CAPTURE_MASK_BLACK: u64 = 18374403900871409664; // B3 - H8 (omitted promotions)
pub const PAWN_H_SIDE_CAPTURE_MASK_BLACK: u64 = 9187201950435704832; // A3- G8 (omitted promotions)


pub const PAWN_A_SIDE_CAPTURE_PROMOTION_MASK_WITHE: u64 = 71494644084506624; // B7 - H7
pub const PAWN_H_SIDE_CAPTURE_PROMOTION_MASK_WITHE: u64 = 35747322042253312; // A7- G7

pub const PAWN_A_SIDE_CAPTURE_PROMOTION_MASK_BLACK: u64 = 65024; // B2 - H2
pub const PAWN_H_SIDE_CAPTURE_PROMOTION_MASK_BLACK: u64 = 32512; // A2 - G2



pub fn generate_pawn_moves(board: &Board, list: &mut MoveList) {

    // 1. Withe
    if board.side_to_move == Color::White {
        let friendly_pawns = board.pieces[PieceType::Pawn as usize][0];
        let opponent_occupied = board.occupied[1];

        // 1.1 Single Push
        let mut single_push_targets = ((friendly_pawns & !(RANK8_MASK | RANK7_MASK)) << 8) & board.empty_squares;

        while single_push_targets > 0 {
            let to = SQUARES[single_push_targets.trailing_zeros() as usize];
            let from = to - 8;
            list.push(Move::new(from, to, MOVE_FLAG_QUIET));
            single_push_targets &= single_push_targets - 1;
        }

        // 1.2 Double Push
        let base_rank_pawns = friendly_pawns & RANK2_MASK;
        let rank3_unblocked = (base_rank_pawns << 8) & board.empty_squares;
        let mut double_push_targets = (rank3_unblocked << 8) & board.empty_squares;

        while double_push_targets > 0 {
            let to = SQUARES[double_push_targets.trailing_zeros() as usize];
            let from = to - 16;
            list.push(Move::new(from, to, MOVE_FLAG_DOUBLE_PAWN));
            double_push_targets &= double_push_targets - 1;
        }

        // 1.3 Captures
        // 1.3.1 A-Side Capture  (omitted promotion captures)
        let mut a_side_capture_targets = (friendly_pawns & PAWN_A_SIDE_CAPTURE_MASK_WITHE) << 7 & opponent_occupied;

        while a_side_capture_targets > 0 {
            let to = SQUARES[a_side_capture_targets.trailing_zeros() as usize];
            let from = to - 7;
            list.push(Move::new(from, to, MOVE_FLAG_CAPTURE));
            a_side_capture_targets &= a_side_capture_targets - 1;
        }

        // 1.3.2 H-Side Capture (omitted promotion captures)
        let mut h_side_capture_targets = (friendly_pawns & PAWN_H_SIDE_CAPTURE_MASK_WITHE) << 9 & opponent_occupied;

        while h_side_capture_targets > 0 {
            let to = SQUARES[h_side_capture_targets.trailing_zeros() as usize];
            let from = to - 9;
            list.push(Move::new(from, to, MOVE_FLAG_CAPTURE));
            h_side_capture_targets &= h_side_capture_targets - 1;
        }

        // 1.4 Promotion
        // 1.4.1 Pushing promotion
        let mut promotion_targets = ((friendly_pawns & RANK7_MASK) << 8) & board.empty_squares;

        while promotion_targets > 0 {
            let to = SQUARES[promotion_targets.trailing_zeros() as usize];
            let from = to - 8;
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_Q));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_R));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_B));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_N));
            promotion_targets &= promotion_targets - 1;
        }

        // 1.4.2 Capturing Promotion
        // 1.4.2.1 A-side capturing promotion
        let mut promotion_targets_a_side_capture = ((friendly_pawns & PAWN_A_SIDE_CAPTURE_PROMOTION_MASK_WITHE) << 7) & board.occupied[1];
        while promotion_targets_a_side_capture > 0 {
            let to = SQUARES[promotion_targets_a_side_capture.trailing_zeros() as usize];
            let from = to - 7;
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_N_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_B_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_R_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_Q_CAP));
            promotion_targets_a_side_capture &= promotion_targets_a_side_capture - 1;
        }

        // 1.4.2.2 H-side capturing promotion
        let mut promotion_targets_h_side_capture = ((friendly_pawns & PAWN_H_SIDE_CAPTURE_PROMOTION_MASK_WITHE) << 9) & board.occupied[1];
        while promotion_targets_h_side_capture > 0 {
            let to = SQUARES[promotion_targets_h_side_capture.trailing_zeros() as usize];
            let from = to - 9;
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_N_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_B_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_R_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_Q_CAP));
            promotion_targets_h_side_capture &= promotion_targets_h_side_capture - 1;
        }

        // 1.5 En Passant
        if let Some(en_passant_target_square) = board.en_passant_target {
            // Check if the target square is on the 6th rank (A6=40 to H6=47)
            if (en_passant_target_square >= Square::A6) && (en_passant_target_square <= Square::H6) {
                let en_passant_target_bb: u64 = 1_u64 << (en_passant_target_square as u64);

                // 1. Check A-Side capture (<< 7, e.g., D5 -> C6)
                let attacker_mask_a_side = (en_passant_target_bb >> 7) & PAWN_A_SIDE_CAPTURE_MASK_WITHE;
                if (attacker_mask_a_side & friendly_pawns) > 0 {
                    let from = en_passant_target_square - 7;
                    list.push(Move::new(from, en_passant_target_square, MOVE_FLAG_EN_PASSANT));
                }

                // 2. Check H-Side capture (<< 9, e.g., B5 -> C6)
                let attacker_mask_h_side = (en_passant_target_bb >> 9) & PAWN_H_SIDE_CAPTURE_MASK_WITHE;
                if (attacker_mask_h_side & friendly_pawns) > 0 {
                    let from = en_passant_target_square - 9;
                    list.push(Move::new(from, en_passant_target_square, MOVE_FLAG_EN_PASSANT));
                }
            }
        }
    // 2. Black
    } else {
        let friendly_pawns = board.pieces[PieceType::Pawn as usize][1];
        let opponent_occupied = board.occupied[0];

        // 2.1 Single Push
        let mut single_push_targets = ((friendly_pawns & !(RANK1_MASK | RANK2_MASK)) >> 8) & board.empty_squares;

        while single_push_targets > 0 {
            let to = SQUARES[single_push_targets.trailing_zeros() as usize];
            let from = to + 8_u8;
            list.push(Move::new(from, to, MOVE_FLAG_QUIET));
            single_push_targets &= single_push_targets - 1;
        }

        // 2.2 Double Push
        let base_rank_pawns = friendly_pawns & RANK7_MASK;
        let rank6_unblocked = (base_rank_pawns >> 8) & board.empty_squares;
        let mut double_push_targets = (rank6_unblocked >> 8) & board.empty_squares;

        while double_push_targets > 0 {
            let to = SQUARES[double_push_targets.trailing_zeros() as usize];
            let from = to + 16_u8;
            list.push(Move::new(from, to, MOVE_FLAG_DOUBLE_PAWN));
            double_push_targets &= double_push_targets - 1;
        }

        // 2.3 Captures
        // 2.3.1 A-Side Capture (>> 9)
        let mut a_side_capture_targets = (friendly_pawns & PAWN_A_SIDE_CAPTURE_MASK_BLACK) >> 9 & opponent_occupied;

        while a_side_capture_targets > 0 {
            let to = SQUARES[a_side_capture_targets.trailing_zeros() as usize];
            let from = to + 9_u8;
            list.push(Move::new(from, to, MOVE_FLAG_CAPTURE));
            a_side_capture_targets &= a_side_capture_targets - 1;
        }

        // 2.3.2 H-Side Capture (>> 7)
        let mut h_side_capture_targets = (friendly_pawns & PAWN_H_SIDE_CAPTURE_MASK_BLACK) >> 7 & opponent_occupied;

        while h_side_capture_targets > 0 {
            let to = SQUARES[h_side_capture_targets.trailing_zeros() as usize];
            let from = to + 7_u8;
            list.push(Move::new(from, to, MOVE_FLAG_CAPTURE));
            h_side_capture_targets &= h_side_capture_targets - 1;
        }

        // 2.4 Promotion
        // 2.4.1 Pushing promotion
        let mut promotion_targets = ((friendly_pawns & RANK2_MASK) >> 8) & board.empty_squares;

        while promotion_targets > 0 {
            let to = SQUARES[promotion_targets.trailing_zeros() as usize];
            let from = to + 8_u8;
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_Q));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_R));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_B));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_N));
            promotion_targets &= promotion_targets - 1;
        }

        // 2.4.2 Capturing Promotion
        // 2.4.2.1 A-side capturing promotion (>> 9)
        let mut promotion_targets_a_side_capture = ((friendly_pawns & PAWN_A_SIDE_CAPTURE_PROMOTION_MASK_BLACK) >> 9) & opponent_occupied;
        while promotion_targets_a_side_capture > 0 {
            let to = SQUARES[promotion_targets_a_side_capture.trailing_zeros() as usize];
            let from = to + 9_u8;
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_N_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_B_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_R_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_Q_CAP));
            promotion_targets_a_side_capture &= promotion_targets_a_side_capture - 1;
        }

        // 2.4.2.2 H-side capturing promotion (>> 7)
        let mut promotion_targets_h_side_capture = ((friendly_pawns & PAWN_H_SIDE_CAPTURE_PROMOTION_MASK_BLACK) >> 7) & opponent_occupied;
        while promotion_targets_h_side_capture > 0 {
            let to = SQUARES[promotion_targets_h_side_capture.trailing_zeros() as usize];
            let from = to + 7_u8;
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_N_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_B_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_R_CAP));
            list.push(Move::new(from, to, MOVE_FLAG_PROMO_Q_CAP));
            promotion_targets_h_side_capture &= promotion_targets_h_side_capture - 1;
        }

        // 2.5 En Passant
        if let Some(en_passant_target_square) = board.en_passant_target {
            // Check if the target square is on the 3rd rank (A3=16 to H3=23)
            if (en_passant_target_square >= Square::A3) && (en_passant_target_square <= Square::H3) {
                let en_passant_target_bb: u64 = 1_u64 << (en_passant_target_square as u64);

                // 1. Check A-Side capture (>> 9, e.g., B4 -> A3)
                let attacker_mask_a_side = (en_passant_target_bb << 9) & PAWN_A_SIDE_CAPTURE_MASK_BLACK;
                if (attacker_mask_a_side & friendly_pawns) > 0 {
                    let from = en_passant_target_square + 9_u8;
                    list.push(Move::new(from, en_passant_target_square, MOVE_FLAG_EN_PASSANT));
                }

                // 2. Check H-Side capture (>> 7, e.g., G4 -> H3)
                let attacker_mask_h_side = (en_passant_target_bb << 7) & PAWN_H_SIDE_CAPTURE_MASK_BLACK;
                if (attacker_mask_h_side & friendly_pawns) > 0 {
                    let from = en_passant_target_square + 7_u8;
                    list.push(Move::new(from, en_passant_target_square, MOVE_FLAG_EN_PASSANT));
                }
            }
        }
    }
}
