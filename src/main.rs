use chess_engine::board::Board;
use chess_engine::movegen::generate_pseudo_legal_moves;
use chess_engine::movegen::legal_check::is_king_attacked;
use chess_engine::r#move::*;
use chess_engine::search::minimax;
use chess_engine::search::minimax::minimax;

fn main() {
    let mut board = Board::from_fen("rnb1kbnr/pppppppp/8/8/8/4q3/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let (opt_move, _) = minimax(&mut board, 5);
    if let Some(mv) = opt_move {
        println!("Found best move: {}", mv)
    } else {
        println!("No moves found")
    }
}
