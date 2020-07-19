use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Initialize array on declaration.
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let xs_noexplicittype = [1, 2, 3, 4, 5];

    // Initialize all the array elements at once.
    let ys: [i32; 500] = [0; 500];
    let ys_bool = [true, true, false];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("array occupies {} bytes", mem::size_of_val(&xs_noexplicittype));
    println!("array occupies {} bytes", mem::size_of_val(&ys_bool));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Compile error: out of bound indexing.
    // println!("{}", xs[5]);
}