// FILENAME: legal_check.rs
use crate::board::{Board, Color, PieceType};
use crate::movegen::tables::{get_bishop_attacks, get_rook_attacks, ATTACKING_PAWNS, KING_ATTACKS, KNIGHT_ATTACKS, MAGICS_BISHOP, MAGICS_ROOK, PREMASKS_BISHOP, PREMASKS_ROOK, RELEVANT_BITS_BISHOP, RELEVANT_BITS_ROOK};
use crate::square::{Square, SQUARES};

/// Checks if the king of the side that is NOT to move is in check
pub fn is_other_king_attacked(board: &Board) -> bool {
    let king = board.pieces[PieceType::King as usize][!board.side_to_move as usize];
    is_square_attacked(board, SQUARES[king.trailing_zeros() as usize], board.side_to_move)
}

// TODO check if castle is legal (squares in between)

/// calculate if a square on the board is attacked by a color
pub fn is_square_attacked(board: &Board, square: Square, color: Color) -> bool {
    // 1. Non sliding
    // 1.1 Pawn
    let pawns = board.pieces[PieceType::Pawn as usize][color as usize];
    if (pawns & ATTACKING_PAWNS[color as usize][square as usize]) != 0 {
        return true;
    }

    // 1.2 Knight
    let knights = board.pieces[PieceType::Knight as usize][color as usize];
    if (knights & KNIGHT_ATTACKS[square as usize]) != 0 {
        return true;
    }

    // 1.3 King
    let king = board.pieces[PieceType::King as usize][color as usize];
    if (king & KING_ATTACKS[square as usize]) != 0 {
        return true;
    }

    // 2. Sliding
    let blockers = board.all_occupied;
    let queens = board.pieces[PieceType::Queen as usize][color as usize];

    // 2.1 Bishop (and part of Queen)
    let premask_bishop = PREMASKS_BISHOP[square as usize];
    let magic_bishop = MAGICS_BISHOP[square as usize];
    let relevant_bits_bishop = RELEVANT_BITS_BISHOP[square as usize];
    let shift_bishop = 64 - relevant_bits_bishop;
    let attack_table_bishop = get_bishop_attacks();
    let magic_index_bishop = ((blockers & premask_bishop).wrapping_mul(magic_bishop)) >> shift_bishop;
    let bishop_attackable_squares = attack_table_bishop[square as usize][magic_index_bishop as usize];

    let bishops_and_queens = board.pieces[PieceType::Bishop as usize][color as usize] | queens;
    if (bishop_attackable_squares & bishops_and_queens) != 0 {
        return true;
    }

    // 2.2 Rooks (and part of Queen)
    let premask_rook = PREMASKS_ROOK[square as usize];
    let magic_rook = MAGICS_ROOK[square as usize];
    let relevant_bits_rook = RELEVANT_BITS_ROOK[square as usize];
    let shift_rook = 64 - relevant_bits_rook;
    let attack_table_rook = get_rook_attacks();
    let magic_index_rook = ((blockers & premask_rook).wrapping_mul(magic_rook)) >> shift_rook;
    let rook_attackable_squares = attack_table_rook[square as usize][magic_index_rook as usize];

    let rooks_and_queens = board.pieces[PieceType::Rook as usize][color as usize] | queens;
    if (rook_attackable_squares & rooks_and_queens) != 0 {
        return true;
    }

    // No attackers found
    false
}