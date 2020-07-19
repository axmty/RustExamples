fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // Integer copy.
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Unit value: {:?}", unit);

    // Unused variable, with underscore prefix to remove warning when compiling.
    let _unused_variable = 3u32;
}