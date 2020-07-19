fn main() {
    use std::mem;

    let color = "green";

    // 'print' borrows 'color'.
    let print = || println!("'color': {}", color);
    print();

    // 'color' can be borrowed immutably again, because the closure
    // only holds an immutable reference to 'color'.
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of 'print'.
    let _color_moved = color;

    let mut count = 0;

    // This closure captures a '&mut count', and borrows 'count'.
    let mut inc = || {
        count += 1;
        println!("'count': {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // At this point, the closure still mutably borrows 'count'.
    // So the following would lead to an error (because of the next 'inc()').
    // let _reborrow = &count;
    inc();

    // The closure no longer needs to borrow '&mut count'.
    let _count_reborrowed = &mut count;

    // 'movable' has a non-copy type.
    let movable = Box::new(3);

    // 'movable' is moved into the closure.
    let consume = || {
        println!("'movable': {:?}", movable);
        mem::drop(movable);
    };

    // 'consume" consumes the variable so this can be called once.
    consume();

    // Error:
    // consume();

    let haystack = vec![1, 2, 3];

    // 'move' keyword forces closure to take ownership of captured variables.
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
}
