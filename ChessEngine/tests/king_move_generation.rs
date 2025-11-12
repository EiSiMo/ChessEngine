use chess_engine::board::Board;
use chess_engine::movegen::non_sliders::generate_king_moves;
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
fn test_king_moves_start_pos_blocked() {
    let fen_standard = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen_standard);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    // King is completely blocked in the start position
    assert_moves_equal(&list.to_string(), "");
}

#[test]
fn test_king_moves_center() {
    let fen = "8/8/8/8/4K3/8/8/8 w - - 0 1"; // King on e4
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    // 8 moves from e4
    assert_moves_equal(&list.to_string(), "e4d3 e4d4 e4d5 e4e3 e4e5 e4f3 e4f4 e4f5");
}

#[test]
fn test_king_moves_corner() {
    let fen = "K7/8/8/8/8/8/8/8 w - - 0 1"; // King on a8
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    // 3 moves from a8
    assert_moves_equal(&list.to_string(), "a8a7 a8b7 a8b8");
}

#[test]
fn test_king_moves_blocked_friendly() {
    // King on d4, surrounded by friendly pawns
    let fen = "8/8/8/3P1P2/3K4/3P1P2/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    // d3, d5, f3, f5 are blocked.
    // c3, c4, c5, e3, e4, e5 are free.
    assert_moves_equal(&list.to_string(), "d4c3 d4c4 d4c5 d4e3 d4e4 d4e5");
}

#[test]
fn test_king_moves_capture_and_blocked() {
    // King on d4
    // Friendly: c3, e5
    // Enemy: c5, e3
    let fen = "8/8/8/2p1P3/3K4/2P1p3/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    // Blocked: c3, e5
    // Captures: c5, e3
    // Empty: c4, d3, d5, e4, f3, f4, f5
    // Note: f3, f4, f5 are valid moves
    assert_moves_equal(&list.to_string(), "d4c4 d4d3 d4d5 d4e4 d4c5 d4e3");
}

#[test]
fn test_king_moves_black() {
    let fen = "8/8/8/8/4k3/8/8/8 b - - 0 1"; // Black king on e4
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    // 8 moves from e4
    assert_moves_equal(&list.to_string(), "e4d3 e4d4 e4d5 e4e3 e4e5 e4f3 e4f4 e4f5");
}

#[test]
fn test_king_moves_castling_white_all() {
    // King on e1, rooks on a1, h1. All rights. No pieces between.
    let fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    // 5 standard moves + 2 castling moves (e1g1, e1c1)
    assert_moves_equal(&list.to_string(), "e1d1 e1d2 e1e2 e1f1 e1f2 O-O O-O-O");
}

#[test]
fn test_king_moves_castling_black_all() {
    // King on e8, rooks on a8, h8. All rights. No pieces between.
    let fen = "r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    // 5 standard moves + 2 castling moves (e8g8, e8c8)
    assert_moves_equal(&list.to_string(), "O-O O-O-O e8d8 e8e7 e8f8 e8d7 e8f7");
}

#[test]
fn test_king_moves_castling_blocked_pieces_white() {
    // White: Queenside blocked by knight
    let fen = "r3k2r/8/8/8/8/8/8/RN2K2R w KQkq - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    // Should only get Kingside castling (e1g1) and standard moves
    assert_moves_equal(&list.to_string(), "e1d1 e1d2 e1e2 e1f1 e1f2 O-O");
}

#[test]
fn test_king_moves_castling_blocked_pieces_black() {
    // Black: Kingside blocked by bishop
    let fen = "r3kb1r/8/8/8/8/8/8/R3K2R b KQkq - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "e8d8 e8d7 e8e7 e8f7 O-O-O");
}

#[test]
fn test_king_moves_castling_no_rights() {
    // Same as `test_king_moves_castling_white_all` but no castling rights
    let fen = "r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    // 5 standard moves, 0 castling
    assert_moves_equal(&list.to_string(), "e1d1 e1d2 e1e2 e1f1 e1f2");
}

#[test]
fn test_king_moves_empty_board() {
    let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_king_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "");
}