use crate::board::*;
use crate::eval::piece_square_tables::PSQT;

pub fn evaluate_board(board: &Board) -> i32 {
    let mut mg_score = 0_i32;
    let mut eg_score = 0_i32;
    let mut phase = 0_i32;

    // We use a macro to force loop unrolling.
    // This enables the compiler to use constant offsets for PSQT access
    // instead of calculating addresses at runtime based on a loop variable.
    macro_rules! score_piece {
        ($pt:expr, $phase_weight:expr) => {
            // --- WHITE ---
            let mut pieces = board.pieces[$pt][Color::White as usize];
            if pieces > 0 {
                // Phase calculation uses count_ones (POPPCNT) which is very fast
                phase += (pieces.count_ones() as i32) * $phase_weight;

                while pieces > 0 {
                    let sq = pieces.trailing_zeros() as usize;
                    pieces &= pieces - 1; // Clear LS1B

                    // Material is already baked into PSQT, so we just add the table value
                    // Since $pt is a const literal here, this compiles to a direct memory access
                    mg_score += PSQT[$pt][Color::White as usize][0][sq];
                    eg_score += PSQT[$pt][Color::White as usize][1][sq];
                }
            }

            // --- BLACK ---
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

    // Explicitly unrolled execution order
    // Pawn (0), Weight 0
    score_piece!(0, 0);
    // Knight (1), Weight 1
    score_piece!(1, 1);
    // Bishop (2), Weight 1
    score_piece!(2, 1);
    // Rook (3), Weight 2
    score_piece!(3, 2);
    // Queen (4), Weight 4
    score_piece!(4, 4);
    // King (5), Weight 0 (Phase doesn't change)
    score_piece!(5, 0);

    // Tapered Evaluation
    let phase = phase.min(24);
    let mg_phase = phase;
    let eg_phase = 24 - phase;

    ((mg_score * mg_phase) + (eg_score * eg_phase)) / 24
}