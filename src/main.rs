use std::error::Error;

use scan::zombie::start::exec;

fn main() -> Result<(), Box<dyn Error>> {
    unsafe { exec() }?;
    Ok(())
}
