// ... (your use statements)
use crate::board::Board;
use crate::r#move::Move;
use crate::search::minimax::minimax;

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

    pub fn search(&mut self, depth: u8) {
        let (opt_move, _score) = minimax(&mut self.board, depth);

        if let Some(mv) = opt_move {
            println!("bestmove {}", mv);
        } else {
            // UCI format for no legal moves (checkmate/stalemate)
            println!("bestmove null");
        }
    }
}