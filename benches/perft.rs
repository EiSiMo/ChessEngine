use chess_engine::board::Board;
use chess_engine::movegen::generate_pseudo_legal_moves;
use chess_engine::movegen::legal_check::is_other_king_attacked;
use chess_engine::r#move::MoveList;
use criterion::{criterion_group, criterion_main, Criterion};

fn count_legal_moves_recursive(board: &mut Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1_u64;
    }
    let mut list = MoveList::new();
    generate_pseudo_legal_moves(&board, &mut list);
    let mut leaf_nodes = 0_u64;
    for mv in list.iter() {
        let undo_info = board.make_move(*mv);
        if !is_other_king_attacked(board) {
            leaf_nodes += count_legal_moves_recursive(board, depth - 1);
        }
        board.undo_move(undo_info);
    }
    leaf_nodes
}


fn run_perft_benchmark(c: &mut Criterion) {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    c.bench_function("standard_perft5", |b| {
        b.iter(|| {
            assert_eq!(count_legal_moves_recursive(&mut board, 5), 4865609);
        })
    });
}

criterion_group!(benches, run_perft_benchmark);
criterion_main!(benches);
