use chess_engine::board::Board;
use chess_engine::movegen::generate_pseudo_legal_moves;
use chess_engine::r#move::*;


fn main() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut move_list = MoveList::new();
    
    generate_pseudo_legal_moves(&board, &mut move_list);

    for mv in move_list.iter() {
        board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        println!("--> making move: {}", mv.to_algebraic());
        board.make_move(*mv);
        board.pretty_print_ascii();
        println!("---------------------");
    }
}
