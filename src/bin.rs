#![warn(clippy::all)]

extern crate failure;
#[macro_use]
extern crate log;

use failure::Error;

fn real_main() -> Result<(), Error> {
    info!("going to print {}", rust_101::foo());
    println!("hello there {}", rust_101::foo());
    println!("is this ok? {}", rust_101::bar()?);

    let s = "String".to_owned();
    let s = s + "s";
    let s2 = &s[1..2];
    println!("value: {}", s2);


    Ok(())
}

fn main() {
    real_main().expect("Error running real main!");
}
// In rust 2018 you can just use `fn real_main() -> Result<(), Error> {`