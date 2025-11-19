use crate::board::{Board, Color};
use crate::eval::evaluate_board;
use crate::movegen::legal_check::*;
use crate::r#move::Move;
use crate::movegen::picker::MoveGenerator;
use crate::tt::{TranspositionTable, NodeType};
use std::time::{Instant, Duration};

const MATE_SCORE: i32 = 1_000_000;

fn evaluate_board_relative(board: &Board) -> i32 {
    let static_eval = evaluate_board(board);
    match board.side_to_move {
        Color::White => static_eval,
        Color::Black => -static_eval,
    }
}

fn score_to_tt(score: i32, ply: u8) -> i32 {
    if score > MATE_SCORE - 1000 {
        score + (ply as i32)
    } else if score < -MATE_SCORE + 1000 {
        score - (ply as i32)
    } else {
        score
    }
}

fn score_from_tt(score: i32, ply: u8) -> i32 {
    if score > MATE_SCORE - 1000 {
        score - (ply as i32)
    } else if score < -MATE_SCORE + 1000 {
        score + (ply as i32)
    } else {
        score
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
    tt: &mut TranspositionTable,
) -> (Option<Move>, i32) {
    if (*nodes).is_multiple_of(4096)
        && start_time.elapsed() > time_limit {
        return (None, 0);
    }
    *nodes += 1;

    let tt_key = board.hash;
    let mut tt_move: Option<Move> = None;

    if let Some(entry) = tt.probe(tt_key) {
        if entry.bm.0 != 0 {
            tt_move = Some(entry.bm);
        }

        if entry.depth >= depth {
            let tt_score = score_from_tt(entry.score, ply);

            match entry.node_type {
                NodeType::Exact => return (Some(entry.bm), tt_score),
                NodeType::Alpha => {
                    if tt_score <= alpha {
                        return (Some(entry.bm), tt_score);
                    }
                }
                NodeType::Beta => {
                    if tt_score >= beta {
                        return (Some(entry.bm), tt_score);
                    }
                }
                _ => {}
            }
        }
    }

    if depth == 0 {
        return (None, evaluate_board_relative(board));
    }

    let mut best_move: Option<Move> = None;
    let mut best_score: i32 = -i32::MAX;
    let mut legal_moves_found = false;
    let alpha_orig = alpha;

    let mut picker = MoveGenerator::new();
    let mut moves_tried = 0;

    loop {
        // Move selection logic:
        // 1. Try TT move first (if exists)
        // 2. Then use the picker for the rest
        let mv = if moves_tried == 0 && tt_move.is_some() {
            tt_move.unwrap()
        } else {
            match picker.next(board) {
                Some(m) => {
                    // Important: skip the TT move if we see it again in the generator
                    if Some(m) == tt_move {
                        continue;
                    }
                    m
                }
                None => break, // No more moves
            }
        };

        moves_tried += 1;

        let undo_mv = board.make_move(mv);

        let is_illegal = is_other_king_attacked(board);
        if is_illegal {
            board.undo_move(undo_mv);
            continue;
        }
        legal_moves_found = true;

        let (_, score) = alpha_beta(board, depth - 1, ply + 1, -beta, -alpha, start_time, time_limit, nodes, tt);

        if (*nodes).is_multiple_of(4096) && start_time.elapsed() > time_limit {
            board.undo_move(undo_mv);
            return (None, 0);
        }

        let current_score = -score;

        if current_score > best_score {
            best_score = current_score;
            best_move = Some(mv);
        }

        board.undo_move(undo_mv);

        if best_score > alpha {
            alpha = best_score;
        }

        if alpha >= beta {
            break;
        }
    }

    if !legal_moves_found {
        if is_current_king_attacked(board) {
            return (None, -MATE_SCORE + (ply as i32));
        } else {
            return (None, 0);
        }
    }

    let node_type = if best_score <= alpha_orig {
        NodeType::Alpha
    } else if best_score >= beta {
        NodeType::Beta
    } else {
        NodeType::Exact
    };

    let save_move = best_move.unwrap_or(Move(0));
    let save_score = score_to_tt(best_score, ply);

    tt.store(tt_key, save_score, depth, node_type, save_move);

    (best_move, best_score)
}