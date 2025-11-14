use crate::board::{Board, Color, PieceType};

pub fn evaluate_board(board: &Board) -> i32 {
    let mut score = 0_i32;
    score += board.pieces[PieceType::Pawn as usize][Color::White as usize].count_ones() as i32 * 100;
    score += board.pieces[PieceType::Knight as usize][Color::White as usize].count_ones() as i32 * 300;
    score += board.pieces[PieceType::Bishop as usize][Color::White as usize].count_ones() as i32 * 300;
    score += board.pieces[PieceType::Rook as usize][Color::White as usize].count_ones() as i32 * 500;
    score += board.pieces[PieceType::Queen as usize][Color::White as usize].count_ones() as i32 * 900;
    score += board.pieces[PieceType::King as usize][Color::White as usize].count_ones() as i32 * 10000;
    score -= board.pieces[PieceType::Pawn as usize][Color::Black as usize].count_ones() as i32 * 100;
    score -= board.pieces[PieceType::Knight as usize][Color::Black as usize].count_ones() as i32 * 300;
    score -= board.pieces[PieceType::Bishop as usize][Color::Black as usize].count_ones() as i32 * 300;
    score -= board.pieces[PieceType::Rook as usize][Color::Black as usize].count_ones() as i32 * 500;
    score -= board.pieces[PieceType::Queen as usize][Color::Black as usize].count_ones() as i32 * 900;
    score -= board.pieces[PieceType::King as usize][Color::Black as usize].count_ones() as i32 * 10000;
    score
}