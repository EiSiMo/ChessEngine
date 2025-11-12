use chess_engine::board::Board;
use chess_engine::movegen::generate_pseudo_legal_moves;
use chess_engine::r#move::*;


fn main() {
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 1 2");
    let mut move_list = MoveList::new();
    
    generate_pseudo_legal_moves(&board, &mut move_list);
    
    println!("Counted {} pseudo legal moves.", move_list.len());
}
