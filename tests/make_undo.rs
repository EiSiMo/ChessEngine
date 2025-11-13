use chess_engine::board::Board;
use chess_engine::movegen::generate_pseudo_legal_moves;
use chess_engine::r#move::MoveList;

/// Helper function to run the make/undo integrity check for any given board state.
/// This avoids code duplication in all the specific test cases.
fn assert_make_undo_integrity(board: &mut Board) {
    let mut list = MoveList::new();
    generate_pseudo_legal_moves(&board, &mut list);

    // If no moves are generated (e.g., stalemate/checkmate), the test passes.
    if list.is_empty() {
        return;
    }

    for mv in list.iter() {
        let board_before_make = board.to_fen();
        let undo_obj = board.make_move(*mv);
        let board_after_make = board.to_fen();

        // Ensure the board actually changed
        assert_ne!(
            board_before_make, board_after_make,
            "Board did not change after make_move for move: {:?}", mv
        );

        board.undo_move(undo_obj);
        let board_after_undo = board.to_fen();

        // Ensure the board is perfectly restored
        assert_eq!(
            board_before_make, board_after_undo,
            "Board state mismatch after undo_move for move: {:?}", mv
        );
    }
}

#[test]
fn test_make_undo_standard() {
    let fen_standard = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(fen_standard);
    assert_make_undo_integrity(&mut board);
}

#[test]
/// Tests a complex, mid-game position with many interactions.
/// This is the famous "Kiwipete" FEN used for perft testing.
fn test_make_undo_kiwipete() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let mut board = Board::from_fen(fen);
    assert_make_undo_integrity(&mut board);
}

#[test]
/// Tests the en passant capture for White (e.g., e5xd6).
/// This ensures the black pawn on d5 is correctly removed and restored.
fn test_make_undo_en_passant_white() {
    // Position after 1. e4 e6 2. e5 d5
    let fen = "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPP2PPP/RNBQKBNR w KQkq d6 0 3";
    let mut board = Board::from_fen(fen);
    assert_make_undo_integrity(&mut board);
}

#[test]
/// Tests the en passant capture for Black (e.g., d5xe6).
/// This ensures the white pawn on e5 is correctly removed and restored.
fn test_make_undo_en_passant_black() {
    // Position after 1. d4 c5 2. d5 e5
    let fen = "rnbqkbnr/pp1p1ppp/8/2pPp3/8/8/PPP1PPPP/RNBQKBNR b KQkq e6 0 3";
    let mut board = Board::from_fen(fen);
    assert_make_undo_integrity(&mut board);
}

#[test]
/// Tests White's kingside (O-O) and queenside (O-O-O) castling.
/// Ensures both rook and king moves are correctly undone.
fn test_make_undo_castling_white() {
    let fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
    let mut board = Board::from_fen(fen);
    assert_make_undo_integrity(&mut board);
}

#[test]
/// Tests Black's kingside (O-O) and queenside (O-O-O) castling.
/// Ensures both rook and king moves are correctly undone.
fn test_make_undo_castling_black() {
    let fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1";
    let mut board = Board::from_fen(fen);
    assert_make_undo_integrity(&mut board);
}

#[test]
/// Tests white pawn promotions (quiet and capture) to Q, R, B, N.
fn test_make_undo_promotions_white() {
    // White pawn on c7, black rooks on b8 and d8 to test capture-promotions
    let fen = "1r1rkb1r/2Ppppb1/8/8/8/8/1PPPPPP1/RNBQKBNR w KQk - 0 1";
    let mut board = Board::from_fen(fen);
    assert_make_undo_integrity(&mut board);
}

#[test]
/// Tests black pawn promotions (quiet and capture) to q, r, b, n.
fn test_make_undo_promotions_black() {
    // Black pawn on g2, white rooks on f1 and h1 to test capture-promotions
    let fen = "RNBQKBNR/1PPPPP2/8/8/8/8/6p1/R4RK1 b Qkq - 0 1";
    let mut board = Board::from_fen(fen);
    assert_make_undo_integrity(&mut board);
}