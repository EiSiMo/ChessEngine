use chess_engine::board::Board;
use chess_engine::movegen::legal_check::is_other_king_attacked;
use chess_engine::movegen::picker::MoveGenerator;

fn count_legal_moves_recursive(board: &mut Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1_u64;
    }

    let mut generator = MoveGenerator::new();
    let mut leaf_nodes = 0_u64;

    while let Some(mv) = generator.next(board) {
        let undo_info = board.make_move(mv);
        if !is_other_king_attacked(board) {
            leaf_nodes += count_legal_moves_recursive(board, depth - 1);
        }
        board.undo_move(undo_info);
    }
    leaf_nodes
}

#[test]
fn test_perft() {
    // Illegal ep move #1
    let mut board = Board::from_fen("3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 1134888, "Illegal ep move #1");

    // Illegal ep move #2
    let mut board = Board::from_fen("8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 1015133, "Illegal ep move #2");

    // EP Capture Checks Opponent
    let mut board = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 1440467, "EP Capture Checks Opponent");

    // Short Castling Gives Check
    let mut board = Board::from_fen("5k2/8/8/8/8/8/8/4K2R w K - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 661072, "Short Castling Gives Check");

    // Long Castling Gives Check
    let mut board = Board::from_fen("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 803711, "Long Castling Gives Check");

    // Castle Rights
    let mut board = Board::from_fen("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 4), 1274206, "Castle Rights");

    // Castling Prevented
    let mut board = Board::from_fen("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 4), 1720476, "Castling Prevented");

    // Promote out of Check
    let mut board = Board::from_fen("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 3821001, "Promote out of Check");

    // Discovered Check
    let mut board = Board::from_fen("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 5), 1004658, "Discovered Check");

    // Promote to give check
    let mut board = Board::from_fen("4k3/1P6/8/8/8/8/K7/8 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 217342, "Promote to give check");

    // Under Promote to give check
    let mut board = Board::from_fen("8/P1k5/K7/8/8/8/8/8 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 92683, "Under Promote to give check");

    // Self Stalemate
    let mut board = Board::from_fen("K1k5/8/P7/8/8/8/8/8 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 2217, "Self Stalemate");

    // Stalemate & Checkmate
    let mut board = Board::from_fen("8/k1P5/8/1K6/8/8/8/8 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 7), 567584, "Stalemate & Checkmate");

    // Stalemate & Checkmate
    let mut board = Board::from_fen("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 4), 23527, "Stalemate & Checkmate");

    // Standard position
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 6), 119060324, "Standard position");

    // Kiwipete
    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 5), 193690690, "Kiwipete");

    // Position 3
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 5), 674624, "Position 3");

    // Position 4
    let mut board = Board::from_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1");
    assert_eq!(count_legal_moves_recursive(&mut board, 5), 15833292, "Position 4");

    // Position 5
    let mut board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    assert_eq!(count_legal_moves_recursive(&mut board, 4), 2103487, "Position 5");

    // Position 6
    let mut board = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
    assert_eq!(count_legal_moves_recursive(&mut board, 5), 164075551, "Position 6");
}