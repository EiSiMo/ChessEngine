use crate::board::Board;
use crate::movegen::non_sliders::{generate_king_moves, generate_knight_moves};
use crate::movegen::pawns::generate_pawn_moves;
use crate::movegen::sliders::{generate_bishop_moves, generate_queen_moves, generate_rook_moves};
use crate::r#move::MoveList;

pub mod non_sliders;
pub mod sliders;
pub mod pawns;
pub mod tables;
pub mod legal_check;

pub fn generate_pseudo_legal_moves(board: &Board, list: &mut MoveList) {
    generate_pawn_moves(board, list);
    generate_knight_moves(board, list);
    generate_bishop_moves(board, list);
    generate_rook_moves(board, list);
    generate_queen_moves(board, list);
    generate_king_moves(board, list);
}