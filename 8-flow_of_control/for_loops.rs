fn main() {
    // 1 inclusive, 101 exclusive.
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 1 inclusive, 100 inclusive.
    // for n in 1..=100 {
    //     if n % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if n % 3 == 0 {
    //         println!("fizz");
    //     } else if n % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", n);
    //     }
    // }

    let names1 = vec!["Bob", "Franck", "Ferris"];
    for name in names1.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // Can access the collection after, iter only borrows each element through
    // each iteration.
    println!("{}", names1[0]);

    let names2 = vec!["Bob", "Franck", "Ferris"];
    for name in names2.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // Error: cannot access since into_inter consumes the collection,
    // it has been moved.
    // println!("{}", names2[0]);

    let mut names3 = vec!["Bob", "Franck", "Ferris"];
    for name in names3.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    // iter_mut mutably borrows each element.
    println!("names: {:?}", names3);
}