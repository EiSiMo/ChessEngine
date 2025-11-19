use crate::board::Board;
use crate::r#move::Move;
use crate::search::alpha_beta::alpha_beta;
use crate::tt::TranspositionTable; // Import TT
use std::time::{Instant, Duration};

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

        // We usually clear the TT or age it before a new search,
        // but for now we keep it to learn from previous moves.
        // self.tt.clear();

        let mut nodes = 0;

        // Initial search at depth 1
        // Note: We pass &mut self.tt to alpha_beta
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

            if start_time.elapsed() > time_limit {
                break;
            }

            opt_move = new_move;
            _score = new_score;

            depth += 1;
        }

        if let Some(mv) = opt_move {
            mv.to_algebraic()
        } else {
            "null".to_string()
        }
    }
}