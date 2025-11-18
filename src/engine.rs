use crate::board::Board;
use crate::r#move::Move;
use crate::search::alpha_beta::alpha_beta;
use std::time::{Instant, Duration};

pub struct Engine {
    pub name: String,
    pub author: String,
    pub board: Board,
}

impl Engine {
    pub fn new(name: String, author: String) -> Engine {
        // Use the standard starting position
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        Engine {
            name,
            author,
            board
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
        
        // We track nodes to limit how often we check the clock inside alpha_beta
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
            &mut nodes
        );
        
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
                &mut nodes
            );

            // If time ran out during the search, alpha_beta returns garbage (None, 0).
            // We must verify we still have time before accepting the new result.
            if start_time.elapsed() > time_limit {
                break; // Discard new_move, keep the one from the previous depth
            }

            opt_move = new_move;
            _score = new_score;
            
            depth += 1;
        }

        if let Some(mv) = opt_move {
            mv.to_algebraic()
        } else {
            // UCI format for no legal moves (checkmate/stalemate)
            "null".to_string()
        }
    }
}
