fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        
        println!("inner short: {}", short_lived_binding);

        // Shadowing the outer binding, within the inner scope.
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // Error: the variable does not exist in this scope.
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';

    // Shadowing within the outer scope.
    println!("outer long: {}", long_lived_binding);
}