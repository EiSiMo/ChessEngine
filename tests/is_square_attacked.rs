use chess_engine::board::{Board, Color};
use chess_engine::movegen::legal_check::is_square_attacked;
use chess_engine::square::Square;


fn assert_square_attacked(board: &mut Board, square: Square, white: bool, black: bool) {
    assert_eq!(is_square_attacked(board, square, Color::White), white, "{}", square);
    assert_eq!(is_square_attacked(board, square, Color::Black), black, "{}", square);
}

#[test]
fn test_is_attacked_kiwipete() {
    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    assert_square_attacked(&mut board, Square::B1, true, false);
    assert_square_attacked(&mut board, Square::C1, true, false);
    assert_square_attacked(&mut board, Square::D1, true, false);
    assert_square_attacked(&mut board, Square::F1, true, false);
    assert_square_attacked(&mut board, Square::G1, true, false);

    assert_square_attacked(&mut board, Square::A3, true, true);
    assert_square_attacked(&mut board, Square::B3, true, false);
    assert_square_attacked(&mut board, Square::D3, true, true);
    assert_square_attacked(&mut board, Square::E3, true, false);
    assert_square_attacked(&mut board, Square::G3, true, false);
    
    // TODO keep going with the rest
}