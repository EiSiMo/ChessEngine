use chess_engine::engine::Engine;
use chess_engine::uci::uci_mainloop;
use chess_engine::zobrist::init_zobrist;

fn main() {
    init_zobrist();
    let mut engine = Engine::new("Yakari".to_string(), "EiSiMo".to_string());
    uci_mainloop(&mut engine);
}
