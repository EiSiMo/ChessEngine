use crate::Engine;

impl Engine {
    pub fn alphabeta(&mut self, position: chess::Board, depth: u8, alpha, beta) -> (Option<chess::ChessMove>, i32) {
        let whites_turn = position.side_to_move() == chess::Color::White;

        if depth == 0 || position.status() != chess::BoardStatus::Ongoing {
            return (None, self.evaluate(position))
        }

        let move_gen = chess::MoveGen::new_legal(&position);

        let mut current_best_move: Option<chess::ChessMove> = None;

        let mut current_best_score = i32::MAX;
        if whites_turn {
            current_best_score = i32::MIN;
        }

        for new_move in move_gen {
            let mut new_position = position.clone();
            position.make_move(new_move, &mut new_position);
            let (_, new_score) = self.minmax(new_position, depth-1);
            if whites_turn {
                if new_score > current_best_score {
                    current_best_score = new_score;
                    current_best_move = Some(new_move);
                }
            } else {
                if new_score < current_best_score {
                    current_best_score = new_score;
                    current_best_move = Some(new_move);
                }
            }
        }
        return (current_best_move, current_best_score)
    }
}