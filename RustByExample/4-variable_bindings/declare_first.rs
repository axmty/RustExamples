fn main() {
    let a_binding;

    {
        let x = 2;

        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error: must initialize before use a variable binding.
    // println!("another binding: {}", another_binding);

    another_binding = 1;
    
    // Now OK.
    println!("another binding: {}", another_binding);
}