use std::fs::File;
use std::io::{self, BufRead};
use chess_engine::engine::Engine;
use std::time::{Instant, Duration};
use chess_engine::zobrist::init_zobrist;

fn load_csv(path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut rows = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let cols = line
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        rows.push(cols);
    }
    Ok(rows)
}

fn main() {
    init_zobrist();
    let time_limit_ms = 1000_u64;
    let time_limit = Duration::from_millis(time_limit_ms);
    
    let mut total_tests: f32 = 0.0;
    let mut correct_tests: f32 = 0.0;
    let sts = load_csv("src/bin/stockfish_testsuite.csv").unwrap();
    let mut engine = Engine::new("Yakari".to_string(), "EiSiMo".to_string());
    
    for test in &sts {
        let fen = &test[0];
        let bm = &test[1];

        engine.setpos_fen(fen);

        let start_time = Instant::now();

        let result = engine.search(time_limit_ms-1);

        let duration = start_time.elapsed();
        if duration > time_limit {
            panic!(
                "Test exceeded 1 second limit: {:?} for FEN: {}",
                duration, fen
            );
        }

        total_tests += 1.0;
        if result == *bm {
            correct_tests += 1.0;
        }

    }

    println!("{}", correct_tests / (total_tests / 100.0));
}
