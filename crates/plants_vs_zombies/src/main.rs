use std::error::Error;
use plants_vs_zombies::start::inject;

fn main() -> Result<(), Box<dyn Error>> {
    unsafe { inject() }?;
    Ok(())
}
