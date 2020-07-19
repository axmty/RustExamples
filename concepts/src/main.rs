const MAX_POINTS: u32 = 100_000;

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    // Cannot assign twice, x is immutable.
    // x = 6;
    // println!("The value of x is: {}", x);

    // Shadowing previous variable x.
    let x = x + MAX_POINTS;
    println!("The value of x is: {}", x);

    // Shadowing and changing type, shadowing just allows to use an existing name.
    let x = x.to_string();
    println!("The value of x is: {}", x);

    // Function parse needs type information to know what type to parse.
    // let guess = "42".parse().expect("Not a number!");
    // OK, type annotation gives the compiler the type information it needs for parse function.
    let _guess: u32 = "42".parse().expect("Not a number!");
    // OK, turbofish gives the same information as type annotation.
    let guess = "42".parse::<u32>().expect("Not a number!");

    println!("{}", guess);

    let _x = 2.0; // f64 by default
    let _y: f32 = 3.0;
    let _sum = 5 + 10; // i32 by default
    let _diff = 83.4 - 23.1; // f64
    let _t = true;
    let _f: bool = false;
    let _c = 'z'; // char is 4 bytes in size (from U+0000 to U+D7FF and U+E000 to U+10FFFF)
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
    let _tuple = (120, 34.3); // inferred to (i32, f64)
    let tuple: (i8, f32) = (120, 34.3);
    let (_x, _y) = tuple; // destructuring with pattern matching
    let _one = tuple.0;
    let _two = tuple.1;
    let _a = [1, 2, 4]; // inferred type is [i32; 3]
    let _a: [u8; 3] = [1, 2, 4];

    println!("{}", other_func(4, 3));
    println!("{}", check_less_five(2));

    let condition = true;
    let _number = if condition { 5 } else { 6 };

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);

    // Same result as previous loop.
    let mut counter = 0;
    while counter < 10 {
        counter += 1;
    }
    let result = counter * 2;
    println!("{}", result);

    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Same as previous for loop.
    for number in 1..6 {
        println!("the value is: {}", number);
    }
}

fn other_func(x: i8, y: i8) -> i8 {
    println!("Other function!");

    // Error, statements do not return values.
    // let a = (let b = 3);
    let a = {
        let b = 3; // this block is an expression and evaluates to 4
        b + 1
    };
    x * y + a
}

fn check_less_five(n: i32) -> bool {
    if n < 5 {
        true
    } else {
        false
    }
    // Could be just write as follow
    // n < 5
}
