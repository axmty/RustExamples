fn main() {
    // Infered type u8, because of suffix annotation.
    let elem = 5u8;

    let mut vec = Vec::new();

    // After that statement, compiler now that vec is a vector of u8.
    vec.push(elem);
    
    println!("{:?}", vec);
}