use chess_engine::board::Board;
use chess_engine::movegen::generate_pseudo_legal_moves;
use chess_engine::r#move::{Move, MoveList, MOVE_FLAG_WK_CASTLE, MOVE_FLAG_WQ_CASTLE, MOVE_FLAG_BK_CASTLE, MOVE_FLAG_BQ_CASTLE};
use chess_engine::square::Square;

fn assert_move_generated(fen:&str, mv: Move, contains: bool) {
    let mut list = MoveList::new();
    let mut board = Board::from_fen(fen);
    generate_pseudo_legal_moves(&mut board, &mut list);
    assert_eq!(list.contains(&mv), contains, "board '{fen}' contains move '{mv}': {contains}")
}

#[test]
fn test_wk_castle_under_attacks() {
    let kingside_castle = Move::new(Square::E1, Square::G1, MOVE_FLAG_WK_CASTLE);
    assert_move_generated("4k3/8/3r4/8/8/8/8/4K2R w K - 0 1", kingside_castle, true);
    assert_move_generated("4k3/8/4r3/8/8/8/8/4K2R w K - 0 1", kingside_castle, false);
    assert_move_generated("4k3/8/5r2/8/8/8/8/4K2R w K - 0 1", kingside_castle, false);
    assert_move_generated("4k3/8/6r1/8/8/8/8/4K2R w K - 0 1", kingside_castle, false);
    assert_move_generated("4k3/8/7r/8/8/8/8/4K2R w K - 0 1", kingside_castle, true);
    assert_move_generated("4k3/8/8/8/8/8/8/r3K2R w K - 0 1", kingside_castle, false);
}

#[test]
fn test_wq_castle_under_attacks() {
    let kingside_castle = Move::new(Square::E1, Square::C1, MOVE_FLAG_WQ_CASTLE);
    assert_move_generated("4k3/8/5r2/8/8/8/8/R3K3 w Q - 0 1", kingside_castle, true);
    assert_move_generated("4k3/8/4r3/8/8/8/8/R3K3 w Q - 0 1", kingside_castle, false);
    assert_move_generated("4k3/8/3r4/8/8/8/8/R3K3 w Q - 0 1", kingside_castle, false);
    assert_move_generated("4k3/8/2r5/8/8/8/8/R3K3 w Q - 0 1", kingside_castle, false);
    assert_move_generated("4k3/8/1r6/8/8/8/8/R3K3 w Q - 0 1", kingside_castle, true);
    assert_move_generated("4k3/8/r7/8/8/8/8/R3K3 w Q - 0 1", kingside_castle, true);
}

#[test]
fn test_bk_castle_under_attacks() {
    let kingside_castle = Move::new(Square::E8, Square::G8, MOVE_FLAG_BK_CASTLE);
    assert_move_generated("4k2r/8/8/8/8/3R4/8/4K3 b k - 0 1", kingside_castle, true);
    assert_move_generated("4k2r/8/8/8/8/4R3/8/4K3 b k - 0 1", kingside_castle, false);
    assert_move_generated("4k2r/8/8/8/8/5R2/8/4K3 b k - 0 1", kingside_castle, false);
    assert_move_generated("4k2r/8/8/8/8/6R1/8/4K3 b k - 0 1", kingside_castle, false);
    assert_move_generated("4k2r/8/8/8/8/7R/8/4K3 b k - 0 1", kingside_castle, true);
    assert_move_generated("2R1k2r/8/8/8/8/8/8/4K3 b k - 0 1", kingside_castle, false);
}

#[test]
fn test_bq_castle_under_attacks() {
    let kingside_castle = Move::new(Square::E8, Square::C8, MOVE_FLAG_BQ_CASTLE);
    assert_move_generated("r3k3/8/8/8/8/5R2/8/4K3 b q - 0 1", kingside_castle, true);
    assert_move_generated("r3k3/8/8/8/8/4R3/8/4K3 b q - 0 1", kingside_castle, false);
    assert_move_generated("r3k3/8/8/8/8/3R4/8/4K3 b q - 0 1", kingside_castle, false);
    assert_move_generated("r3k3/8/8/8/8/2R5/8/4K3 b q - 0 1", kingside_castle, false);
    assert_move_generated("r3k3/8/8/8/8/1R6/8/4K3 b q - 0 1", kingside_castle, true);
    assert_move_generated("r3k3/8/8/8/8/R7/8/4K3 b q - 0 1", kingside_castle, true);
}