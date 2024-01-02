use crate::Engine;

impl Engine {
    pub fn minmax(&mut self, position: chess::Board, depth: u8) -> (Option<chess::ChessMove>, i32) {
        if depth == 0 || position.status() != chess::BoardStatus::Ongoing {
            return (None, self.evaluate(position))
        }

        let mut move_gen = chess::MoveGen::new_legal(&position);

        let mut current_best_move = move_gen.next().unwrap();

        let mut new_position = position.clone();
        position.make_move(current_best_move, &mut new_position);

        let (_, mut current_best_score) = self.minmax(new_position, depth-1);

        for new_move in move_gen {
            new_position = position.clone();
            position.make_move(new_move, &mut new_position);
            let (_, new_score) = self.minmax(new_position, depth-1);
            if position.side_to_move() == chess::Color::White {
                if new_score > current_best_score {
                    current_best_score = new_score;
                    current_best_move = new_move;
                }
            } else {
                if new_score < current_best_score {
                    current_best_score = new_score;
                    current_best_move = new_move;
                }
            }
        }
        return (Some(current_best_move), current_best_score)
    }
}