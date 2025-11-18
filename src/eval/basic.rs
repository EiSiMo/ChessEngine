use crate::board::*;
use crate::eval::piece_square_tables::PSQT;

// Pawn, Knight, Bishop, Rook, Queen
pub const MATERIAL_WEIGHTS: [i32; 5] = [100, 300, 300, 500, 900];
pub const PHASE_WEIGHTS: [i32; 5] = [0, 1, 1, 2, 4];

pub fn evaluate_board(board: &Board) -> i32 {
    let mut mg_score = 0_i32;
    let mut eg_score = 0_i32;
    let mut phase = 0_i32;

    // --- WHITE PIECES ---
    // Iterating Pawn (0) to Queen (4) for Material + Phase + PSQT
    for pt in 0..5 {
        let mut pieces = board.pieces[pt][Color::White as usize];
        let count = pieces.count_ones() as i32;
        
        mg_score += count * MATERIAL_WEIGHTS[pt];
        eg_score += count * MATERIAL_WEIGHTS[pt];
        phase += count * PHASE_WEIGHTS[pt];

        while pieces > 0 {
            let sq = pieces.trailing_zeros() as usize;
            pieces &= pieces - 1; // Clear LS1B
            
            // Access: [Piece][Color][Phase (0=MG, 1=EG)][Square]
            mg_score += PSQT[pt][Color::White as usize][0][sq];
            eg_score += PSQT[pt][Color::White as usize][1][sq];
        }
    }

    // King (Index 5) - No Material/Phase weight, only PSQT
    let mut white_king = board.pieces[5][Color::White as usize];
    if white_king > 0 {
        let sq = white_king.trailing_zeros() as usize;
        mg_score += PSQT[5][Color::White as usize][0][sq];
        eg_score += PSQT[5][Color::White as usize][1][sq];
    }

    // --- BLACK PIECES ---
    // Iterating Pawn (0) to Queen (4)
    for pt in 0..5 {
        let mut pieces = board.pieces[pt][Color::Black as usize];
        let count = pieces.count_ones() as i32;

        mg_score -= count * MATERIAL_WEIGHTS[pt];
        eg_score -= count * MATERIAL_WEIGHTS[pt];
        phase += count * PHASE_WEIGHTS[pt];

        while pieces > 0 {
            let sq = pieces.trailing_zeros() as usize;
            pieces &= pieces - 1;

            mg_score -= PSQT[pt][Color::Black as usize][0][sq];
            eg_score -= PSQT[pt][Color::Black as usize][1][sq];
        }
    }

    // King (Index 5) for Black
    let mut black_king = board.pieces[5][Color::Black as usize];
    if black_king > 0 {
        let sq = black_king.trailing_zeros() as usize;
        mg_score -= PSQT[5][Color::Black as usize][0][sq];
        eg_score -= PSQT[5][Color::Black as usize][1][sq];
    }

    // Tapered Evaluation Interpolation
    let phase = phase.min(24); // Clamp to 24 max
    let mg_phase = phase;
    let eg_phase = 24 - phase;

    ((mg_score * mg_phase) + (eg_score * eg_phase)) / 24
}
