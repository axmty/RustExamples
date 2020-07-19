// Aliases to give a name to existing types.
type NanoSecond = u64;
type Inch = u64;

// If non camel case alias, compiler raises a warning.
#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // No extra type safety, aliases are not new types.
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches);
}