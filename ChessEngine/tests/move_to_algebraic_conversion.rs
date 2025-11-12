use chess_engine::r#move::*;
use chess_engine::square::Square;

#[test]
fn test_quiet_move_white_pawn() {
    // Test 1: Standard Quiet Move (White Pawn)
    // (from: E2, to: E4)
    // NOTE: This was MOVE_TYPE_FLAG_QUIET, but in the new system it's a specific flag.
    // The algebraic notation is the same, so we test with the new specific flag.
<<<<<<< HEAD
    let m_quiet = Move::new(Square::E2, Square::E4, MOVE_FLAG_QUIET);
=======
    let m_quiet = Move::new(Square::E2, Square::E4, MOVE_FLAG_DOUBLE_PAWN);
>>>>>>> origin/master
    assert_eq!(m_quiet.to_algebraic(), "e2e4");
}

#[test]
fn test_quiet_move_black_knight() {
    // Test 2: Standard Quiet Move (Black Knight)
    // (from: B8, to: C6)
    let m_knight = Move::new(Square::B8, Square::C6, MOVE_FLAG_QUIET);
    assert_eq!(m_knight.to_algebraic(), "b8c6");
}

#[test]
fn test_en_passant_move() {
    // Test 3: En Passant Move (Notation is same as quiet move)
    // (from: E5, to: F6)
    let m_ep = Move::new(Square::E5, Square::F6, MOVE_FLAG_EN_PASSANT);
    assert_eq!(m_ep.to_algebraic(), "e5f6");
}

#[test]
fn test_promotion_to_queen() {
    // Test 4: Promotion to Queen (Push)
    // (from: E7, to: E8)
    let m_promo_q = Move::new(Square::E7, Square::E8, MOVE_FLAG_PROMO_Q);
    assert_eq!(m_promo_q.to_algebraic(), "e7e8q");
}

#[test]
fn test_promotion_to_rook() {
    // Test 5: Promotion to Rook (Push)
    // (from: A7, to: A8)
    let m_promo_r = Move::new(Square::A7, Square::A8, MOVE_FLAG_PROMO_R);
    assert_eq!(m_promo_r.to_algebraic(), "a7a8r");
}

#[test]
fn test_promotion_to_bishop() {
    // Test 6: Promotion to Bishop (Capture)
    // (from: G2, to: H1)
    let m_promo_b = Move::new(Square::G2, Square::H1, MOVE_FLAG_PROMO_CAP_B);
    assert_eq!(m_promo_b.to_algebraic(), "g2h1b");
}

#[test]
fn test_promotion_to_knight() {
    // Test 7: Promotion to Knight (Capture)
    // (from: G7, to: F8)
    let m_promo_n = Move::new(Square::G7, Square::F8, MOVE_FLAG_PROMO_CAP_N);
    assert_eq!(m_promo_n.to_algebraic(), "g7f8n");
}

#[test]
fn test_white_kingside_castling() {
    // Test 8: White Kingside Castling
    // (from: E1, to: G1)
<<<<<<< HEAD
    let m_castle_wk = Move::new(Square::E1, Square::G1, MOVE_FLAG_WK_CASTLE);
=======
    let m_castle_wk = Move::new(Square::E1, Square::G1, MOVE_FLAG_KING_CASTLE);
>>>>>>> origin/master
    assert_eq!(m_castle_wk.to_algebraic(), "O-O");
}

#[test]
fn test_white_queenside_castling() {
    // Test 9: White Queenside Castling
    // (from: E1, to: C1)
<<<<<<< HEAD
    let m_castle_wq = Move::new(Square::E1, Square::C1, MOVE_FLAG_WQ_CASTLE);
=======
    let m_castle_wq = Move::new(Square::E1, Square::C1, MOVE_FLAG_QUEEN_CASTLE);
>>>>>>> origin/master
    assert_eq!(m_castle_wq.to_algebraic(), "O-O-O");
}

#[test]
fn test_black_kingside_castling() {
    // Test 10: Black Kingside Castling
    // (from: E8, to: G8)
<<<<<<< HEAD
    let m_castle_bk = Move::new(Square::E8, Square::G8, MOVE_FLAG_BK_CASTLE);
=======
    let m_castle_bk = Move::new(Square::E8, Square::G8, MOVE_FLAG_KING_CASTLE);
>>>>>>> origin/master
    assert_eq!(m_castle_bk.to_algebraic(), "O-O");
}

#[test]
fn test_black_queenside_castling() {
    // Test 11: Black Queenside Castling
    // (from: E8, to: C8)
<<<<<<< HEAD
    let m_castle_bq = Move::new(Square::E8, Square::C8, MOVE_FLAG_BQ_CASTLE);
=======
    let m_castle_bq = Move::new(Square::E8, Square::C8, MOVE_FLAG_QUEEN_CASTLE);
>>>>>>> origin/master
    assert_eq!(m_castle_bq.to_algebraic(), "O-O-O");
}
