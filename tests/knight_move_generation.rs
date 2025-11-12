use chess_engine::board::Board;
use chess_engine::movegen::non_sliders::generate_knight_moves;
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
fn test_knight_move_generation() {
    let fen_standard = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen_standard);
    let mut list = MoveList::new();
    generate_knight_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "b1a3 b1c3 g1f3 g1h3");
}

#[test]
fn test_knight_move_generation_black() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_knight_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "b8a6 b8c6 g8f6 g8h6");
}

#[test]
fn test_knight_moves_center() {
    let fen = "8/8/8/3N4/8/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_knight_moves(&board, &mut list);
    // 8 moves from d5
    assert_moves_equal(&list.to_string(), "d5b4 d5b6 d5c3 d5c7 d5e3 d5e7 d5f4 d5f6");
}

#[test]
fn test_knight_moves_capture() {
    // Black knight on e5, white pawn on d3
    let fen = "8/8/8/4n3/8/3P4/8/8 b - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_knight_moves(&board, &mut list);
    // 8 moves, including capture on d3
    assert_moves_equal(&list.to_string(), "e5c4 e5c6 e5d3 e5d7 e5f3 e5f7 e5g4 e5g6");
}

#[test]
fn test_knight_moves_blocked_friendly() {
    // Knight on d4, some moves are blocked, some are not.
    let fen = "8/8/3P1P2/3P1P2/3N4/3P1P2/3P1P2/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_knight_moves(&board, &mut list);
    // f3, f5, d3, d5, d6, f6 are blocked.
    // c2, e2, b3, b5, c6, e6 are free.
    assert_moves_equal(&list.to_string(), "d4c2 d4e2 d4b3 d4b5 d4c6 d4e6");
}

#[test]
fn test_knight_moves_corner() {
    let fen = "N7/8/8/8/8/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_knight_moves(&board, &mut list);
    // 2 moves from a8
    assert_moves_equal(&list.to_string(), "a8b6 a8c7");
}

#[test]
fn test_knight_moves_capture_and_blocked() {
    // White knights on b1 and g1.
    // b1: a3 (friendly), c3 (friendly), d2 (enemy)
    // g1: f3 (empty), h3 (empty), e2 (empty)
    let fen = "8/8/8/8/8/P1P5/3p4/1N4NR w K - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_knight_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "b1d2 g1e2 g1f3 g1h3");
}

#[test]
fn test_knight_moves_empty_board() {
    let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
    let board = Board::from_fen(fen);
    let mut list = MoveList::new();
    generate_knight_moves(&board, &mut list);
    assert_moves_equal(&list.to_string(), "");
}