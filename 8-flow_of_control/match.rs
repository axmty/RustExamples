// Not all variants of Color are used.
#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime!"),
        13 ..= 19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    
    // Match is an expression too.
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    // Destructuring tuples.
    let pair = (0, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (0, y) => println!("First is '0' and 'y' is {:?}", y),
        (x, 0) => println!("'x' is {:?} and last is '0'", x),
        _ => println!("It doesn't matter what they are"),
    }

    // Enums.
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, and value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, and lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, and yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, and key: {}!", c, m, y, k),
    }

    // Assign a reference of type i32.
    let reference = &4;
    
    // If the two & are dropped, then 'val' should be assigned a value.
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the &, we can dereference 'reference' before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }


    let _not_a_ref = 3;

    // Same as 'let _is_a_ref = &3'.
    let ref _is_a_ref = 3;


    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. 'mut_value': {:?}", m);
        }
    }


    // Struct destructuring.
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y }
            => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: i }
            => println!("y is 2, i = {:?}", i),
        Foo { y, .. }
            => println!("y = {}, we don't care about x", y),
        // Error: pattern does not mention field 'x'.
        // Foo { y } => println!("y = {}", y);
    }


    // Guards.
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first on is odd"),
        _ => println!("No correlation..."),
    }


    // Binding.
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I'm not born yet I guess"),
        n @ 1 ..= 12  => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        n             => println!("I'm an old person of age {:?}", n),
    }


    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n)      => println!("Not interesting... {}", n),
        _            => (),
    }
}




