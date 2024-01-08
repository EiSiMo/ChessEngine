use crate::Engine;

impl Engine {
    pub fn alphabeta(&mut self, position: chess::Board, depth: u8, mut alpha: i32, mut beta: i32) -> (Option<chess::ChessMove>, i32) {
        if depth == 0 || position.status() != chess::BoardStatus::Ongoing {
            return (None, self.evaluate(position))
        }

        let whites_turn = position.side_to_move() == chess::Color::White;

        let move_gen = chess::MoveGen::new_legal(&position);

        let mut current_best_move: Option<chess::ChessMove> = None;
        let mut current_best_score: i32 = 0;

        if whites_turn {

            for new_move in move_gen {
                current_best_score = i32::MIN;
                let mut new_position = position.clone();
                position.make_move(new_move, &mut new_position);
                let (_, new_score) = self.minmax(new_position, depth-1);
                if new_score > current_best_score {
                    current_best_score = new_score;
                    current_best_move = Some(new_move);
                }

                if alpha > new_score {
                    alpha = new_score;
                }
                if beta <= alpha {
                    break;
                }
            }
        } else {
            current_best_score = i32::MAX;
            for new_move in move_gen {
                let mut new_position = position.clone();
                position.make_move(new_move, &mut new_position);
                let (_, new_score) = self.minmax(new_position, depth-1);

                if new_score < current_best_score {
                    current_best_score = new_score;
                    current_best_move = Some(new_move);
                }

                if beta < new_score {
                    beta = new_score;
                }
                if beta <= alpha {
                    break;
                }
            }
        }
        return (current_best_move, current_best_score)
    }
}