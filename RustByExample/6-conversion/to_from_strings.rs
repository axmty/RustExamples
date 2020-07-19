use std::fmt;
use std::str::FromStr;
use std::num;

struct Circle {
    radius: i32,
}

// Implementing the fmt::Display trait provides an implementation of the
// ToString trait.
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

// See https://doc.rust-lang.org/std/str/trait.FromStr.html for an example.
impl FromStr for Circle {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let radius = s.parse::<i32>()?;

        Ok(Circle { radius })
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    // Use Circle::from_str(&str).
    let c = Circle::from_str("4");
    println!("{}", c.unwrap());
}