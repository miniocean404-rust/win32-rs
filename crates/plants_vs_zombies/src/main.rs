use game_dll::inject::ptr_scan;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // unsafe { inject() }?;
    unsafe { ptr_scan() }?;
    Ok(())
}
