<<<<<<< HEAD
use chess_engine::board::*;
=======
use chess_engine::board::Board;
>>>>>>> origin/master

#[test]
fn test_fen_roundtrip_standard() {
    let fen_standard = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    assert_eq!(Board::from_fen(fen_standard).to_fen(), fen_standard);
}

#[test]
fn test_fen_roundtrip_kiwipete() {
    let fen_kiwipete = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    assert_eq!(Board::from_fen(fen_kiwipete).to_fen(), fen_kiwipete);
}

#[test]
fn test_fen_roundtrip_en_passant() {
    let fen_en_passant = "rnbqkbnr/pppppp1p/8/8/p7/4P3/PPPP1PPP/RNBQKBNR w KQkq e3 0 1";
    assert_eq!(Board::from_fen(fen_en_passant).to_fen(), fen_en_passant);
}

#[test]
fn test_fen_roundtrip_castle() {
    let fen_castle = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2QK2R b - - 0 1";
    assert_eq!(Board::from_fen(fen_castle).to_fen(), fen_castle);
}

#[test]
fn test_fen_roundtrip_just_kings() {
    let fen_just_kings = "8/k7/8/8/8/8/7K/8 w - - 0 1";
    assert_eq!(Board::from_fen(fen_just_kings).to_fen(), fen_just_kings);
}

#[test]
fn test_fen_roundtrip_high_move_values() {
    let fen_high_move_values = "8/P1k5/K7/8/8/8/8/8 w - - 0 78";
    assert_eq!(Board::from_fen(fen_high_move_values).to_fen(), fen_high_move_values);
}

#[test]
fn test_fen_roundtrip_empty_count1() {
    let fen_empty_count1 = "1n6/8/8/8/8/8/8/8 w - - 0 1";
    assert_eq!(Board::from_fen(fen_empty_count1).to_fen(), fen_empty_count1);
}

#[test]
fn test_fen_roundtrip_empty_count2() {
    let fen_empty_count2 = "6n1/8/8/8/8/8/8/8 w - - 0 1";
    assert_eq!(Board::from_fen(fen_empty_count2).to_fen(), fen_empty_count2);
}

#[test]
fn test_board_fen_state() {
    let fen_standard = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen_standard);
<<<<<<< HEAD
    assert_eq!(board.pieces[PieceType::Pawn as usize][Color::White as usize], 65280);
    assert_eq!(board.pieces[PieceType::Pawn as usize][Color::Black as usize], 71776119061217280);
    assert_eq!(board.pieces[PieceType::Knight as usize][Color::White as usize], 66);
    assert_eq!(board.pieces[PieceType::Knight as usize][Color::Black as usize], 4755801206503243776);
    assert_eq!(board.pieces[PieceType::Bishop as usize][Color::White as usize], 36);
    assert_eq!(board.pieces[PieceType::Bishop as usize][Color::Black as usize], 2594073385365405696);
    assert_eq!(board.pieces[PieceType::Rook as usize][Color::White as usize], 129);
    assert_eq!(board.pieces[PieceType::Rook as usize][Color::Black as usize], 9295429630892703744);
    assert_eq!(board.pieces[PieceType::Queen as usize][Color::White as usize], 8);
    assert_eq!(board.pieces[PieceType::Queen as usize][Color::Black as usize], 576460752303423488);
    assert_eq!(board.pieces[PieceType::King as usize][Color::White as usize], 16);
    assert_eq!(board.pieces[PieceType::King as usize][Color::Black as usize], 1152921504606846976);
=======
    assert_eq!(board.pawns[0], 65280);
    assert_eq!(board.pawns[1], 71776119061217280);
    assert_eq!(board.knights[0], 66);
    assert_eq!(board.knights[1], 4755801206503243776);
    assert_eq!(board.bishops[0], 36);
    assert_eq!(board.bishops[1], 2594073385365405696);
    assert_eq!(board.rooks[0], 129);
    assert_eq!(board.rooks[1], 9295429630892703744);
    assert_eq!(board.queens[0], 8);
    assert_eq!(board.queens[1], 576460752303423488);
    assert_eq!(board.kings[0], 16);
    assert_eq!(board.kings[1], 1152921504606846976);
>>>>>>> origin/master

    assert_eq!(board.occupied[0], 65535);
    assert_eq!(board.occupied[1], 18446462598732840960);
    assert_eq!(board.all_occupied, 18446462598732906495);

    assert_eq!(board.castling_rights, 15);
    assert_eq!(board.en_passant_target, None);
    assert_eq!(board.halfmove_clock, 0);
    assert_eq!(board.fullmove_number, 1);
}