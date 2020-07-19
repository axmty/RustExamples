fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // Expression assigned to 'y'.
        x_cube + x_squared + x
    };

    // () is assigned to 'z1' and 'z2'. The two blocks are equivalent.
    let z1 = {
        2 * x;
    };

    let z2 = {
        2 * x;
        ()
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z1 and z2 are {:?} and {:?}", z1, z2);
}