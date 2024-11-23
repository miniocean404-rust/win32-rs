use game_dll::start::read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // unsafe { inject() }?;
    unsafe { read() }?;
    Ok(())
}
