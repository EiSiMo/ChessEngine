use chess_engine::board::Board;
use chess_engine::movegen::pawns::generate_pawn_moves;
use chess_engine::r#move::MoveList;

/// Compares two move list strings ignoring the order of moves.
fn assert_moves_equal(actual_str: &str, expected_str: &str) {
    let mut actual_moves: Vec<&str> = actual_str.split_whitespace().collect();
    let mut expected_moves: Vec<&str> = expected_str.split_whitespace().collect();

    actual_moves.sort();
    expected_moves.sort();

    assert_eq!(actual_moves, expected_moves);
}

#[test]
fn test_pawn_moves_start_pos() {
    let fen_standard = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen_standard);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "a2a3 a2a4 b2b3 b2b4 c2c3 c2c4 d2d3 d2d4 e2e3 e2e4 f2f3 f2f4 g2g3 g2g4 h2h3 h2h4");
}

#[test]
fn test_pawn_moves_black_start_pos() {
    let fen_standard = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
    let board = Board::from_fen(fen_standard);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "a7a6 a7a5 b7b6 b7b5 c7c6 c7c5 d7d6 d7d5 e7e6 e7e5 f7f6 f7f5 g7g6 g7g5 h7h6 h7h5");
}

#[test]
fn test_pawn_moves_black_blocked_and_captures() {
    // Black pawns on a7, c7, e7, g7. White pawns on b6, d6, f6.
    let fen = "8/p1p1p1p1/1P1P1P2/8/8/8/8/8 b - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "a7a6 a7a5 a7b6 c7c6 c7c5 c7b6 c7d6 e7e6 e7e5 e7d6 e7f6 g7g6 g7g5 g7f6");
}

#[test]
fn test_pawn_moves_black_side_captures() {
    // Test captures on A and H files (no wrap-around)
    let fen = "8/8/8/8/8/1p4p1/P6P/8 b - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "b3b2 b3a2 g3g2 g3h2");
}

#[test]
fn test_pawn_moves_white_side_captures() {
    // Test captures on A and H files (no wrap-around)
    let fen = "8/p6p/1P4P1/8/8/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "b6b7 b6a7 g6g7 g6h7");
}

#[test]
fn test_pawn_moves_black_en_passant() {
    // White just moved e2e4, en passant target is e3. Black pawn on d4.
    let fen = "rnbqkbnr/ppp1pppp/8/8/3pP3/8/PPP2PPP/RNBQKBNR b KQkq e3 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "a7a6 a7a5 b7b6 b7b5 c7c6 c7c5 d4d3 d4e3 e7e6 e7e5 f7f6 f7f5 g7g6 g7g5 h7h6 h7h5");
}

#[test]
fn test_pawn_moves_white_en_passant() {
    // Black just moved d7d5, en passant target is d6. White pawn on e5.
    let fen = "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "a2a3 a2a4 b2b3 b2b4 c2c3 c2c4 d2d3 d2d4 e5e6 e5d6 f2f3 f2f4 g2g3 g2g4 h2h3 h2h4");
}

#[test]
fn test_pawn_moves_black_promotion() {
    let fen = "8/8/8/8/8/8/p1p1p1p1/8 b - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), 
        "a2a1q a2a1r a2a1b a2a1n c2c1q c2c1r c2c1b c2c1n e2e1q e2e1r e2e1b e2e1n g2g1q g2g1r g2g1b g2g1n");
}

#[test]
fn test_pawn_moves_white_promotion() {
    let fen = "8/P1P1P1P1/8/8/8/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), 
        "a7a8q a7a8r a7a8b a7a8n c7c8q c7c8r c7c8b c7c8n e7e8q e7e8r e7e8b e7e8n g7g8q g7g8r g7g8b g7g8n");
}

#[test]
fn test_pawn_moves_black_promotion_capture() {
    // Black pawn on b2. White rooks on a1 and c1.
    let fen = "8/8/8/8/8/8/1p6/R1R5 b - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), 
        "b2b1q b2b1r b2b1b b2b1n b2a1q b2a1r b2a1b b2a1n b2c1q b2c1r b2c1b b2c1n");
}

#[test]
fn test_pawn_moves_white_promotion_capture() {
    // White pawn on a7. Black rooks on b8 and d8.
    let fen = "1r1r4/P7/8/8/8/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), 
        "a7a8q a7a8r a7a8b a7a8n a7b8q a7b8r a7b8b a7b8n");
}

#[test]
fn test_pawn_moves_black_midgame_random() {
    let fen = "r1bqkb1r/1p2pppp/p1n2n2/3p4/2BNP3/2N5/PPP2PPP/R1BQK2R b KQkq - 1 6";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "a6a5 b7b6 b7b5 d5c4 d5e4 e7e6 e7e5 g7g6 g7g5 h7h6 h7h5");
}

#[test]
fn test_pawn_moves_white_midgame_random() {
    let fen = "r1bqk2r/2ppbppp/p1n2n2/1p2p3/B1P1P3/5N2/PP1P1PPP/RNBQR1K1 w kq - 0 7";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_pawn_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "a2a3 b2b3 b2b4 c4c5 c4b5 d2d3 d2d4 g2g3 g2g4 h2h3 h2h4");
}
