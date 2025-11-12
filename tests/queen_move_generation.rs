use chess_engine::board::Board;
use chess_engine::movegen::sliders::generate_queen_moves;
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
fn test_queen_moves_single_rook() {
    let fen_standard = "8/1Q2p3/8/8/8/8/6p1/8 w - - 0 1";
    let board = Board::from_fen(fen_standard);
    let mut list = MoveList::new();
    generate_queen_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "b7a8 b7a7 b7a6 b7b8 b7b6 b7b5 b7b4 b7b3 b7b2 b7b1 b7c8 b7c7 b7c6 b7d7 b7e7 b7d5 b7e4 b7f3 b7g2");
}

#[test]
fn test_queen_center_open() {
    // Queen in the center, open board, should generate 27 moves
    let fen = "4k3/8/8/8/3Q4/8/8/4K3 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_queen_moves(&board, &mut list);
    let expected = "d4a1 d4b2 d4c3 d4e5 d4f6 d4g7 d4h8 \
                    d4a4 d4b4 d4c4 d4e4 d4f4 d4g4 d4h4 \
                    d4d1 d4d2 d4d3 d4d5 d4d6 d4d7 d4d8 \
                    d4a7 d4b6 d4c5 d4e3 d4f2 d4g1";
    assert_moves_equal(&list.to_string(), expected);
}

#[test]
fn test_queen_corner_blocked_friendly() {
    // Queen in corner, completely blocked by friendly pieces
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PP1PP1PP/QNBQKBNR w Kkq - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_queen_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "d1a4 d1b3 d1c2");
}

#[test]
fn test_queen_multiple_captures_black() {
    // Black queen on h8, with multiple white pieces to capture
    let fen = "q3k3/P1P1P1P1/8/8/8/P1P1P1P1/8/4K3 b - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_queen_moves(&board, &mut list);
    let expected = "a8b8 a8c8 a8d8 a8a7 a8b7 a8c6 a8d5 a8e4 a8f3 a8g2 a8h1";
    assert_moves_equal(&list.to_string(), expected);
}

#[test]
fn test_multiple_queens() {
    let fen = "4k3/8/8/8/8/8/8/Q3K2Q w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_queen_moves(&board, &mut list);
    let expected = "a1a2 a1a3 a1a4 a1a5 a1a6 a1a7 a1a8 a1b1 a1c1 a1d1 a1b2 a1c3 a1d4 a1e5 a1f6 a1g7 a1h8 \
                    h1h2 h1h3 h1h4 h1h5 h1h6 h1h7 h1h8 h1g1 h1f1 h1g2 h1f3 h1e4 h1d5 h1c6 h1b7 h1a8";
    assert_moves_equal(&list.to_string(), expected);
}

#[test]
fn test_queen_rook_only() {
    // Queen on d4, bishop moves blocked by friendly pawns
    let fen = "4k3/8/8/2P1P3/3Q4/2P1P3/8/4K3 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_queen_moves(&board, &mut list);
    let expected = "d4a4 d4b4 d4c4 d4e4 d4f4 d4g4 d4h4 \
                    d4d1 d4d2 d4d3 d4d5 d4d6 d4d7 d4d8";
    assert_moves_equal(&list.to_string(), expected);
}

#[test]
fn test_queen_bishop_only() {
    // Queen on d4, rook moves blocked by friendly pawns
    let fen = "4k3/8/8/3P4/2PQP3/3P4/8/4K3 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_queen_moves(&board, &mut list);
    let expected = "d4a1 d4b2 d4c3 d4e5 d4f6 d4g7 d4h8 \
                    d4a7 d4b6 d4c5 d4e3 d4f2 d4g1";
    assert_moves_equal(&list.to_string(), expected);
}
