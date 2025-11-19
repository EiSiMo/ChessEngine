use crate::board::*;
use crate::psqt::PSQT;

pub fn evaluate_board(board: &Board) -> i32 {
    let mut mg_score = 0_i32;
    let mut eg_score = 0_i32;
    let mut phase = 0_i32;

    // We use a macro to force loop unrolling.
    macro_rules! score_piece {
        ($pt:expr, $phase_weight:expr) => {
            let mut pieces = board.pieces[$pt][Color::White as usize];
            if pieces > 0 {
                phase += (pieces.count_ones() as i32) * $phase_weight;

                while pieces > 0 {
                    let sq = pieces.trailing_zeros() as usize;
                    pieces &= pieces - 1; // Clear LS1B

                    mg_score += PSQT[$pt][Color::White as usize][0][sq];
                    eg_score += PSQT[$pt][Color::White as usize][1][sq];
                }
            }

            let mut pieces = board.pieces[$pt][Color::Black as usize];
            if pieces > 0 {
                phase += (pieces.count_ones() as i32) * $phase_weight;

                while pieces > 0 {
                    let sq = pieces.trailing_zeros() as usize;
                    pieces &= pieces - 1;

                    mg_score -= PSQT[$pt][Color::Black as usize][0][sq];
                    eg_score -= PSQT[$pt][Color::Black as usize][1][sq];
                }
            }
        };
    }

    score_piece!(0, 0);
    score_piece!(1, 1);
    score_piece!(2, 1);
    score_piece!(3, 2);
    score_piece!(4, 4);
    score_piece!(5, 0);

    let phase = phase.min(24);
    let mg_phase = phase;
    let eg_phase = 24 - phase;

    ((mg_score * mg_phase) + (eg_score * eg_phase)) / 24
}