mod state;

use state::State;
use std::{error, thread::sleep, time::Duration};

fn main() -> Result<(), Box<dyn error::Error>> {
    let fp = format!("{}/assets/glider.grid", env!("CARGO_MANIFEST_DIR"));
    let mut state = State::load(&fp)?;
    let dur = Duration::from_millis(50);
    loop {
        println!("{state}");
        sleep(dur);
        state.advance();
        println!();
    }
}
