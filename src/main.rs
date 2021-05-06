use std::time::SystemTime;

mod types;
mod enums;
mod helpers;


fn main() {

    let start_time = SystemTime::now();

    let mut b = types::board::Board::default();
    let depth = 9_u8;
    let amount = 20_u8;
    for _ in 0..amount {
        b = b.move_best(depth);
        println!("{}", b.as_fen())
    }

    match start_time.elapsed() {
       Ok(elapsed) => {
           println!("Generating {} moves with depth {} took {} ms", amount, depth, elapsed.as_millis());
       }
       Err(e) => {
           // an error occurred!
           println!("Error: {:?}", e);
       }
   }
}
