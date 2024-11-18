use std::error::Error;
use scan::app::index::run;

fn main() -> Result<(), Box<dyn Error>> {
    unsafe { run() }?;
    Ok(())
}
