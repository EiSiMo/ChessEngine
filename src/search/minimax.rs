use crate::board::{Board, Color}; // <-- Assuming you have a Color enum (e.g., Color::White, Color::Black)
use crate::eval::basic::evaluate_board;
use crate::movegen::generate_pseudo_legal_moves;
use crate::movegen::legal_check::is_other_king_attacked;
use crate::r#move::{Move, MoveList};


fn evaluate_board_relative(board: &Board) -> i32 {
    let static_eval = evaluate_board(board);
    match board.side_to_move {
        Color::White => static_eval,
        Color::Black => -static_eval,
    }
}

pub fn minimax(board: &mut Board, depth: u8) -> (Option<Move>, i32) {
    if depth == 0 {
        return (None, evaluate_board_relative(board));
    }

    let mut list = MoveList::new();
    generate_pseudo_legal_moves(board, &mut list);
    let mut best_move: Option<Move> = None;
    let mut best_score: i32 = -i32::MAX;
    let mut legal_moves_found = false;

    for mv in list.iter() {
        let undo_mv = board.make_move(*mv);
        let is_illegal = is_other_king_attacked(board);
        if is_illegal {
            board.undo_move(undo_mv);
            continue;
        }
        legal_moves_found = true;
        let (_, score) = minimax(board, depth - 1);
        let current_score = -score;

        if current_score > best_score {
            best_score = current_score;
            best_move = Some(*mv);
        }

        board.undo_move(undo_mv);
    }

    if !legal_moves_found {
        if is_other_king_attacked(board) {
            return (None, -i32::MAX);
        } else {
            return (None, 0);
        }
    }

    (best_move, best_score)
}