use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// Define a conversion for own type Number.
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

/*
Implementation of Into provided thanks to the implementation of From above.

impl Number for From<i32> {
    fn into(self) -> Number {
        Number { value: self }
    }
}
*/

fn main() {
    let my_str = "hello";
    let _my_string = String::from(my_str);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    // Using the Into trait, which is the reciprocal of the From<i32> trait.
    // Need to type annotate num2, so that the compiler is able to determine
    // the type to convert from.
    let int = 5;
    let num2: Number = int.into();
    println!("My number is {:?}", num2);
}