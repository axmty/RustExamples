#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // The following would break the inner loop.
            // break;

            // The following breaks the outer loop.
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}