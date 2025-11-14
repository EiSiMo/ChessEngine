use chess_engine::board::Board;
use chess_engine::movegen::generate_pseudo_legal_moves;
use chess_engine::movegen::legal_check::is_king_attacked;
use chess_engine::r#move::*;


fn count_legal_moves_recursive(board: &mut Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1_u64;
    }

    let mut list = MoveList::new();
    generate_pseudo_legal_moves(&board, &mut list);

    let mut leaf_nodes = 0_u64;
    for mv in list.iter() {
        let undo_info = board.make_move(*mv);
        if !is_king_attacked(board) {
            leaf_nodes += count_legal_moves_recursive(board, depth - 1);
        }
        board.undo_move(undo_info);
    }
    leaf_nodes
}

fn perft() {
    // TalkChess PERFT Tests (by Martin Sedlak)
    //--Illegal ep move #1
    let mut board = Board::from_fen("3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 1134888);
    
    //--Illegal ep move #2
    let mut board = Board::from_fen("8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 1015133);
    
    //--EP Capture Checks Opponent
    let mut board = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 1440467);
    
    //--Short Castling Gives Check
    let mut board = Board::from_fen("5k2/8/8/8/8/8/8/4K2R w K - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 661072);
    
    //--Long Castling Gives Check
    let mut board = Board::from_fen("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 803711);
    
    //--Castle Rights
    let mut board = Board::from_fen("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 4), 1274206);
    
    //--Castling Prevented
    let mut board = Board::from_fen("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 4), 1720476);
    
    //--Promote out of Check
    let mut board = Board::from_fen("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 3821001);
    
    //--Discovered Check
    let mut board = Board::from_fen("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 5), 1004658);
    
    //--Promote to give check
    let mut board = Board::from_fen("4k3/1P6/8/8/8/8/K7/8 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 217342);
    
    //--Under Promote to give check
    let mut board = Board::from_fen("8/P1k5/K7/8/8/8/8/8 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 92683);
    
    //--Self Stalemate
    let mut board = Board::from_fen("K1k5/8/P7/8/8/8/8/8 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 2217);
    
    //--Stalemate & Checkmate
    let mut board = Board::from_fen("8/k1P5/8/1K6/8/8/8/8 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 7), 567584);
    
    //--Stalemate & Checkmate
    let mut board = Board::from_fen("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 4), 23527);
    
    // Standard position
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 3), 8902);
}

fn main() {
    perft();
}
