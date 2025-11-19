use chess_engine::board::Board;
use criterion::{criterion_group, criterion_main, Criterion};
use chess_engine::eval::evaluate_board;

fn run_eval_benchmark(c: &mut Criterion) {
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    c.bench_function("standard_board_evaluation", |b| {
        b.iter(|| {
            evaluate_board(&board);
        })
    });
}

criterion_group!(benches, run_eval_benchmark);
criterion_main!(benches);
