use chess_engine::board::Board;
use chess_engine::movegen::sliders::generate_bishop_moves;
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
fn test_bishop_moves_single() {
    let fen_standard = "8/8/8/1p1p4/2B5/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen_standard);
    let mut list = MoveList::new();
    generate_bishop_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "c4b5 c4d5 c4b3 c4d3 c4a2 c4e2 c4f1");
}

#[test]
fn test_bishop_moves_empty_board_center() {
    // Läufer in der Mitte (e4) auf einem leeren Brett
    let fen = "8/8/8/8/4B3/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_bishop_moves(&board, &mut list);
    
    let expected = "e4d3 e4c2 e4b1 e4f5 e4g6 e4h7 e4d5 e4c6 e4b7 e4a8 e4f3 e4g2 e4h1";
    assert_moves_equal(&list.to_string(), expected);
}

#[test]
fn test_bishop_moves_from_corner() {
    // Läufer in der Ecke (a1) auf einem leeren Brett
    let fen = "8/8/8/8/8/8/8/B7 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_bishop_moves(&board, &mut list);
    
    let expected = "a1b2 a1c3 a1d4 a1e5 a1f6 a1g7 a1h8";
    assert_moves_equal(&list.to_string(), expected);
}

#[test]
fn test_bishop_moves_friendly_blockers() {
    // Läufer auf c1, blockiert von EIGENEN Bauern auf b2 und d2.
    // Darf b2/d2 NICHT schlagen und nicht darüber springen.
    let fen = "8/8/8/8/8/8/1P1P4/2B5 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_bishop_moves(&board, &mut list);
    
    // Es sollten KEINE Züge generiert werden.
    assert_moves_equal(&list.to_string(), "");
}

#[test]
fn test_bishop_moves_mixed_blockers_and_captures() {
    let fen = "8/8/1p3P2/8/3B4/8/1p3P2/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_bishop_moves(&board, &mut list);
    
    let expected = "d4c3 d4e3 d4b2 d4c5 d4b6 d4e5";
    assert_moves_equal(&list.to_string(), expected);
}

#[test]
fn test_bishop_moves_black_turn() {
    // Schwarzer Läufer auf c5, weiße Bauern auf b4 und d4.
    let fen = "8/8/8/2b5/1P1P4/8/8/8 b - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_bishop_moves(&board, &mut list);
    
    // Schlagzüge: b4, d4
    // Ruhige Züge: b6, a7, d6, e7, f8
    let expected = "c5b4 c5d4 c5b6 c5a7 c5d6 c5e7 c5f8";
    assert_moves_equal(&list.to_string(), expected);
}
