use chess_engine::board::Board;
use chess_engine::movegen::generate_pseudo_legal_moves;
use chess_engine::movegen::legal_check::is_king_attacked;
use chess_engine::r#move::MoveList;

fn count_legal_moves_recursive(board: &mut Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1_u64;
    }

    let mut list = MoveList::new();
    generate_pseudo_legal_moves(&board, &mut list);

    let mut leaf_nodes = 0_u64;
    for mv in list.iter() {
        // Store the undo info when making the move
        let undo_info = board.make_move(*mv);

        if !is_king_attacked(board) {
            leaf_nodes += count_legal_moves_recursive(board, depth - 1);
        }

        // Undo the move to restore the board state for the next iteration
        board.undo_move(undo_info);
    }
    leaf_nodes
}

#[test]
fn perft() {
    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    println!("{}", count_legal_moves_recursive(&mut board, 1));
}


// "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" 7 3195901860 "false"
// "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -" 5 193690690 "false"
// "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -" 7 178633661 "false"
// "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1" 6 706045033 "false"
// "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8" 5 89941194 "false"
// "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10" 5 164075551 "false"
// "r7/4p3/5p1q/3P4/4pQ2/4pP2/6pp/R3K1kr w Q - 1 3" 5 11609488 "false"