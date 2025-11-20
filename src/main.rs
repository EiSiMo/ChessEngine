use chess_engine::engine::Engine;
use chess_engine::uci::UCI;

fn main() {
    let engine = Engine::new("Yakari".to_string(), "EiSiMo".to_string());
    let mut uci = UCI { engine };
    uci.uci_mainloop();
}
