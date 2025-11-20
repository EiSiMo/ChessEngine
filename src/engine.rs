use crate::board::Board;
use crate::r#move::Move;
use crate::search::alpha_beta;
use crate::tt::TranspositionTable; // Import TT
use std::time::{Instant, Duration};
use crate::uci::UCI;

pub struct Engine {
    pub name: String,
    pub author: String,
    pub board: Board,
    pub tt: TranspositionTable, // Engine owns the TT
}

impl Engine {
    pub fn new(name: String, author: String) -> Engine {
        // Use the standard starting position
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let tt = TranspositionTable::new(4096);

        Engine {
            name,
            author,
            board,
            tt,
        }
    }

    pub fn setpos_fen(&mut self, fen: &str) {
        self.board = Board::from_fen(fen);
    }

    pub fn setpos_startpos(&mut self, moves: &[&str]) {
        self.board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        for mv_str in moves {
            let mv = Move::from_algebraic(mv_str, &self.board);
            self.board.make_move(mv);
        }
    }

    pub fn search(&mut self, time_limit_ms: u64) -> String {
        let start_time = Instant::now();
        let time_limit = Duration::from_millis(time_limit_ms);

        let mut nodes = 0;

        // Initial search at depth 1
        let (mut opt_move, mut _score) = alpha_beta(
            &mut self.board,
            1,
            0,
            -i32::MAX,
            i32::MAX,
            start_time,
            time_limit,
            &mut nodes,
            &mut self.tt
        );

        // If we timed out immediately at depth 1 (very rare), opt_move might be None.
        // But usually, we have at least one move here.

        let mut depth = 2;

        // Iterative Deepening
        while start_time.elapsed() < time_limit {
            let (new_move, new_score) = alpha_beta(
                &mut self.board,
                depth,
                0,
                -i32::MAX,
                i32::MAX,
                start_time,
                time_limit,
                &mut nodes,
                &mut self.tt
            );

            // FIX: Only update if we actually got a move back (didn't timeout)
            if let Some(mv) = new_move {
                opt_move = Some(mv);
                _score = new_score;

                // Optional: Move send_info here to update the GUI after every completed depth
                UCI::send_info(Some(depth as usize), Some(nodes), None, None, None, None);
            } else {
                // If new_move is None, the search was aborted due to time.
                // We discard the partial results and stop increasing depth.
                break;
            }

            depth += 1;
        }

        if let Some(mv) = opt_move {
            mv.to_algebraic()
        } else {
            // Fallback: If even depth 1 failed (e.g. 0ms time limit), try to return *any* legal move
            // or just return null if truly nothing works.
            "null".to_string()
        }
    }
}