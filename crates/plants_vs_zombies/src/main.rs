use std::error::Error;
use plants_vs_zombies::start::exec;

fn main() -> Result<(), Box<dyn Error>> {
    unsafe { exec() }?;
    Ok(())
}
