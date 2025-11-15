use chess_engine::engine::Engine;
use chess_engine::uci::uci_mainloop;

fn main() {
    let mut engine = Engine::new("Yakari".to_string(), "EiSiMo".to_string());
    uci_mainloop(&mut engine);
}
