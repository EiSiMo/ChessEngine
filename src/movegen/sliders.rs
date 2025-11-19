use crate::board::*;
use crate::r#move::*;
use crate::square::SQUARES;
use super::tables::*;

pub fn generate_rook_moves(board: &Board, list: &mut MoveList) {
    let mut friendly_rooks = board.pieces[PieceType::Rook as usize][board.side_to_move as usize];
    while friendly_rooks > 0 {
        let square_index = friendly_rooks.trailing_zeros() as usize;

        let premask = PREMASKS_ROOK[square_index];
        let magic = MAGICS_ROOK[square_index];
        let relevant_bits = RELEVANT_BITS_ROOK[square_index];
        let shift = 64 - relevant_bits;
        let blockers = board.all_occupied & premask;
        let attack_table = get_rook_attacks();

        let magic_index = (blockers.wrapping_mul(magic)) >> shift;

        let movable_squares = attack_table[square_index][magic_index as usize];

        // 1. Normal moves
        let mut quiet_moves = movable_squares & !board.all_occupied;
        while quiet_moves > 0 {
            let from = SQUARES[square_index];
            let to = SQUARES[quiet_moves.trailing_zeros() as usize];
            list.push(Move::new(from, to, MOVE_FLAG_QUIET));
            quiet_moves &= quiet_moves - 1;
        }

        // 2. Captures
        let mut capture_moves = movable_squares & board.occupied[!board.side_to_move as usize];
        while capture_moves > 0 {
            let from = SQUARES[square_index];
            let to = SQUARES[capture_moves.trailing_zeros() as usize];
            list.push(Move::new(from, to, MOVE_FLAG_CAPTURE));
            capture_moves &= capture_moves - 1;
        }

        friendly_rooks &= friendly_rooks - 1;
    }
}

pub fn generate_bishop_moves(board: &Board, list: &mut MoveList) {
    let mut friendly_bishops = board.pieces[PieceType::Bishop as usize][board.side_to_move as usize];
    while friendly_bishops > 0 {
        let square_index = friendly_bishops.trailing_zeros() as usize;

        let premask = PREMASKS_BISHOP[square_index];
        let magic = MAGICS_BISHOP[square_index];
        let relevant_bits = RELEVANT_BITS_BISHOP[square_index];
        let shift = 64 - relevant_bits;
        let blockers = board.all_occupied & premask;
        let attack_table = get_bishop_attacks();

        let magic_index = (blockers.wrapping_mul(magic)) >> shift;

        let movable_squares = attack_table[square_index][magic_index as usize];

        // 1. Normal moves
        let mut quiet_moves = movable_squares & !board.all_occupied;
        while quiet_moves > 0 {
            let from = SQUARES[square_index];
            let to = SQUARES[quiet_moves.trailing_zeros() as usize];
            list.push(Move::new(from, to, MOVE_FLAG_QUIET));
            quiet_moves &= quiet_moves - 1;
        }

        // 2. Captures
        let mut capture_moves = movable_squares & board.occupied[!board.side_to_move as usize];
        while capture_moves > 0 {
            let from = SQUARES[square_index];
            let to = SQUARES[capture_moves.trailing_zeros() as usize];
            list.push(Move::new(from, to, MOVE_FLAG_CAPTURE));
            capture_moves &= capture_moves - 1;
        }

        friendly_bishops &= friendly_bishops - 1;
    }
}

pub fn generate_queen_moves(board: &Board, list: &mut MoveList) {
    let mut friendly_queens = board.pieces[PieceType::Queen as usize][board.side_to_move as usize];
    while friendly_queens > 0 {
        let square_index = friendly_queens.trailing_zeros() as usize;

        // --- 1. Get Rook Attacks ---
        let rook_premask = PREMASKS_ROOK[square_index];
        let rook_magic = MAGICS_ROOK[square_index];
        let rook_relevant_bits = RELEVANT_BITS_ROOK[square_index];
        let rook_shift = 64 - rook_relevant_bits;
        let rook_blockers = board.all_occupied & rook_premask;
        let rook_attack_table = get_rook_attacks();
        let rook_magic_index = (rook_blockers.wrapping_mul(rook_magic)) >> rook_shift;
        let rook_moves = rook_attack_table[square_index][rook_magic_index as usize];

        // --- 2. Get Bishop Attacks ---
        let bishop_premask = PREMASKS_BISHOP[square_index];
        let bishop_magic = MAGICS_BISHOP[square_index];
        let bishop_relevant_bits = RELEVANT_BITS_BISHOP[square_index];
        let bishop_shift = 64 - bishop_relevant_bits;
        let bishop_blockers = board.all_occupied & bishop_premask;
        let bishop_attack_table = get_bishop_attacks();
        let bishop_magic_index = (bishop_blockers.wrapping_mul(bishop_magic)) >> bishop_shift;
        let bishop_moves = bishop_attack_table[square_index][bishop_magic_index as usize];
        
        // --- 3. Combine Attacks ---
        let movable_squares = rook_moves | bishop_moves;

        // --- 4. Generate Moves (Identical to Rook/Bishop) ---
        
        // 4a. Normal moves
        let mut quiet_moves = movable_squares & !board.all_occupied;
        while quiet_moves > 0 {
            let from = SQUARES[square_index];
            let to = SQUARES[quiet_moves.trailing_zeros() as usize];
            list.push(Move::new(from, to, MOVE_FLAG_QUIET));
            quiet_moves &= quiet_moves - 1;
        }

        // 4b. Captures
        let mut capture_moves = movable_squares & board.occupied[!board.side_to_move as usize];
        while capture_moves > 0 {
            let from = SQUARES[square_index];
            let to = SQUARES[capture_moves.trailing_zeros() as usize];
            list.push(Move::new(from, to, MOVE_FLAG_CAPTURE));
            capture_moves &= capture_moves - 1;
        }

        friendly_queens &= friendly_queens - 1;
    }
}
