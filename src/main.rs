mod state;

use state::State;
use std::{error, thread::sleep, time::Duration};

fn main() -> Result<(), Box<dyn error::Error>> {
    let fp = format!("{}/assets/sample.grid", env!("CARGO_MANIFEST_DIR"));
    let mut state = State::load(&fp)?;
    println!("{state}");
    for _ in 0.. {
        sleep(Duration::from_millis(200));
        state.advance();
        println!();
        println!("{state}");
    }
    Ok(())
}
