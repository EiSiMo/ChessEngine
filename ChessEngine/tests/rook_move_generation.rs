use chess_engine::board::Board;
use chess_engine::movegen::sliders::generate_rook_moves;
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
<<<<<<< HEAD
fn test_rook_moves_single_rook() {
=======
fn test_rook_moves_1() {
>>>>>>> origin/master
    let fen_standard = "8/8/8/2b5/2Rb4/2b5/8/8 w - - 0 1";
    let board = Board::from_fen(fen_standard);
    let mut list = MoveList::new();
    generate_rook_moves(&board, &mut list);
<<<<<<< HEAD
    // This FEN has a White Rook at c4, and Black Bishops at c5, d4, and c3.
    // It should be able to capture all three and move left to a4/b4.
    assert_moves_equal(&list.to_string(), "c4a4 c4b4 c4c3 c4c5 c4d4");
}

#[test]
fn test_rook_moves_empty_board() {
    let fen = "8/8/8/8/3R4/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_rook_moves(&board, &mut list);
    let expected = "d4d1 d4d2 d4d3 d4d5 d4d6 d4d7 d4d8 d4a4 d4b4 d4c4 d4e4 d4f4 d4g4 d4h4";
    assert_moves_equal(&list.to_string(), expected);
}

#[test]
fn test_rook_moves_corner_blocked_black() {
    let fen = "r6k/1p6/8/8/8/8/8/K7 b - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_rook_moves(&board, &mut list);
    // Black rook at a8. Friendly king at h8, friendly pawn at b7.
    // Rook can move down the a-file and right along the 8th rank, stopping before h8.
    let expected = "a8a7 a8a6 a8a5 a8a4 a8a3 a8a2 a8a1 a8b8 a8c8 a8d8 a8e8 a8f8 a8g8";
    assert_moves_equal(&list.to_string(), expected);
}

#[test]
fn test_rook_moves_double_rooks_friendly_block() {
    let fen = "8/8/8/8/R3R3/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_rook_moves(&board, &mut list);
    // Rooks at a4 and e4. They block each other horizontally.
    // Rook a4 moves a1-a8 and b4, c4, d4.
    // Rook e4 moves e1-e8, f4, g4, h4 AND d4, c4, b4.
    let expected = "a4a1 a4a2 a4a3 a4a5 a4a6 a4a7 a4a8 a4b4 a4c4 a4d4 \
                    e4e1 e4e2 e4e3 e4e5 e4e6 e4e7 e4e8 e4f4 e4g4 e4h4 \
                    e4d4 e4c4 e4b4";
    assert_moves_equal(&list.to_string(), expected);
}

#[test]
fn test_rook_moves_capture_stops_movegen() {
    let fen = "r7/P7/8/8/8/8/8/8 b - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_rook_moves(&board, &mut list);
    // Black rook at a8, White pawn at a7.
    // The rook can capture at a7, but cannot move past it.
    // It can still move horizontally.
    let expected = "a8a7 a8b8 a8c8 a8d8 a8e8 a8f8 a8g8 a8h8";
    assert_moves_equal(&list.to_string(), expected);
}

#[test]
fn test_rook_moves_completely_blocked_friendly() {
    let fen = "8/8/8/1P6/PRP5/1P6/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_rook_moves(&board, &mut list);
    // White rook at b4.
    // Blocked by P(b5), P(b3), P(a4), P(c4).
    // Should have 0 moves.
    assert_moves_equal(&list.to_string(), "");
}

#[test]
fn test_rook_moves_ignores_absolute_pin() {
    let fen = "r3k3/8/8/8/R3K3/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_rook_moves(&board, &mut list);
    // White rook at a4 is absolutely pinned to King at e4 by Black rook at a8.
    // A pseudo-legal generator should *ignore* the pin.
    // It should generate vertical moves (including capture at a8)
    // and horizontal moves (stopping before the friendly King at e4).
    let expected = "a4a1 a4a2 a4a3 a4a5 a4a6 a4a7 a4a8 a4b4 a4c4 a4d4";
    assert_moves_equal(&list.to_string(), expected);
}
=======
    // King is completely blocked in the start position
    assert_moves_equal(&list.to_string(), "");
}
>>>>>>> origin/master
