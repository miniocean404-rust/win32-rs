use game_dll::start::inject;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    unsafe { inject() }?;
    Ok(())
}
