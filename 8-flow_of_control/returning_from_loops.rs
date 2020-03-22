fn main() {
    let mut counter = 0;

    // loops are expressions. We can return inside with the break statement.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}