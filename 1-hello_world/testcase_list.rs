// Implement fmt::Display to a list structure.
use std::fmt;

// Structure that holds a Vec of int.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Create a reference to the first (and unique) element of our List.
        let vec = &self.0;

        // Since write! returns a fmt::Result, we need to deal with all
        // the cases (equivalent to a try).
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        // No semicolon = return statement.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}