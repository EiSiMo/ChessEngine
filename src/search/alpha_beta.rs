use crate::board::{Board, Color};
use crate::eval::basic::evaluate_board;
use crate::movegen::generate_pseudo_legal_moves;
use crate::movegen::legal_check::*;
use crate::r#move::{Move, MoveList};
use std::time::{Instant, Duration};

// A score high enough to be > any material eval, but low enough to not overflow when adding ply
const MATE_SCORE: i32 = 1_000_000;

fn evaluate_board_relative(board: &Board) -> i32 {
    let static_eval = evaluate_board(board);
    match board.side_to_move {
        Color::White => static_eval,
        Color::Black => -static_eval,
    }
}

pub fn alpha_beta(
    board: &mut Board,
    depth: u8,
    ply: u8,
    mut alpha: i32,
    beta: i32,
    start_time: Instant,
    time_limit: Duration,
    nodes: &mut u64,
) -> (Option<Move>, i32) {
    // Check for time usage every 4096 nodes to reduce system call overhead
    if *nodes % 4096 == 0 {
        if start_time.elapsed() > time_limit {
            // Return immediately. The return value here effectively signals an abort,
            // but the engine must discard this result.
            return (None, 0); 
        }
    }
    *nodes += 1;

    if depth == 0 {
        return (None, evaluate_board_relative(board));
    }

    let mut list = MoveList::new();
    generate_pseudo_legal_moves(board, &mut list);
    let mut best_move: Option<Move> = None;
    let mut best_score: i32 = -i32::MAX; // This is our local "worst case"
    let mut legal_moves_found = false;

    for mv in list.iter() {
        let undo_mv = board.make_move(*mv);
        let is_illegal = is_other_king_attacked(board);
        if is_illegal {
            board.undo_move(undo_mv);
            continue;
        }
        legal_moves_found = true;

        // Recursive call with negated and swapped alpha/beta
        // Pass time parameters and node counter down
        let (_, score) = alpha_beta(board, depth - 1, ply + 1, -beta, -alpha, start_time, time_limit, nodes);
        
        // If we aborted deeper in the tree (returned 0 due to timeout), 
        // we should technically propagate that up, but checking elapsed() 
        // at the loop start (via recursion) handles it eventually.
        // For a strict abort, we check here too:
        if *nodes % 4096 == 0 && start_time.elapsed() > time_limit {
             board.undo_move(undo_mv);
             return (None, 0);
        }

        let current_score = -score;

        if current_score > best_score {
            best_score = current_score;
            best_move = Some(*mv);
        }

        board.undo_move(undo_mv);

        // Alpha-Beta Pruning logic
        if best_score > alpha {
            alpha = best_score;
        }

        if alpha >= beta {
            break; // Beta cutoff (Pruning)
        }
    }

    if !legal_moves_found {
        if is_current_king_attacked(board) {
            return (None, -MATE_SCORE + (ply as i32));
        } else {
            // Stalemate
            return (None, 0);
        }
    }

    (best_move, best_score)
}
