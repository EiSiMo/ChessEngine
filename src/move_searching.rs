use std::cmp::{max, min};
use rand::seq::SliceRandom;

use crate::board::Board;

impl Board {
    pub fn move_random(&mut self) -> Board {
        let possible_moves = self.generate_possible_moves();
        let random_move = possible_moves.choose(&mut rand::thread_rng());
        *random_move.unwrap()
    }

    pub fn minimax(&mut self, depth: u8, mut alpha: i32, mut beta: i32) -> (Board, i32) {
        if depth == 0 || self.is_lost() {
            let score = self.evaluate();
            return (*self, score)
        }

        let mut possible_moves = self.generate_possible_moves();

        // TODO handle index out of bounds
        let mut best_score = possible_moves[0].evaluate();
        let mut best_move = possible_moves[0];

        if self.withes_turn {
            for mut possible_move in possible_moves {
                let result = possible_move.minimax(depth - 1, alpha, beta);
                let eval = result.1;
                if eval > best_score {
                    best_score = eval;
                    best_move = possible_move;
                }
                alpha = max(eval, alpha);
                if beta <= alpha {
                    break;
                }
            }
            (best_move, best_score)
        } else {
            for mut possible_move in possible_moves {
                let result = possible_move.minimax(depth - 1, alpha, beta);
                let eval = result.1;
                if eval < best_score {
                    best_score = eval;
                    best_move = possible_move;
                }
                beta = min(eval, beta);
                if beta <= alpha {
                    break;
                }
            }
            (best_move, best_score)
        }
    }
}