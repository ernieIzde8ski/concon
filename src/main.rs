mod state;

use state::State;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let fp = format!("{}/assets/sample.grid", env!("CARGO_MANIFEST_DIR"));
    let grid = State::load(&fp)?;
    println!("{grid}");
    Ok(())
}
