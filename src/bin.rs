#![warn(clippy::all)]
#[allow(unused_imports)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

use failure::Error;

fn main() -> Result<(), Error> {
    info!("going to print {}", rust_101::foo());
    println!("hello there {}", rust_101::foo());
    println!("is this ok? {}", rust_101::bar()?);
    Ok(())
}

