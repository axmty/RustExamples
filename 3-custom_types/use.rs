// Hide warnings for unused code when compiling.
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Solider,
}

fn main() {
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    // Can use Poor and Civilian thanks to previous 'use' statements.
    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Solider => println!("Soldiers fight!"),
    }
}