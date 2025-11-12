pub mod non_sliders;
pub mod sliders;
pub mod pawns;
pub mod tables;

use crate::board::Board;
use crate::r#move::*;
use non_sliders::*;
use sliders::*;
use pawns::*;

pub fn generate_pseudo_legal_moves(board: &Board, list: &mut MoveList) {
    generate_pawn_moves(board, list);
    generate_knight_moves(board, list);
    generate_bishop_moves(board, list);
    generate_rook_moves(board, list);
    generate_queen_moves(board, list);
    generate_king_moves(board, list);
}