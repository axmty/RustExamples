#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error: no implicit conversion.
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // Casting to unsigned types.
    println!("1000 ({0:>032b}) as a u16 is: {1:>4} ({1:>016b})", 1000, 1000 as u16);
    println!("1000 ({0:>032b}) as a  i8 is: {1:>4} ({1:>08b})", 1000, 1000 as u8);
    println!("  -1 ({0:>032b}) as a  u8 is: {1:>4} ({1:>08b})", -1, -1i8 as u8);

    // Casting to signed types.
    println!(" 128 ({0:>032b}) as a i16 is: {1:>4} ({1:>016b})", 128, 128 as i16);
    println!(" 128 ({0:>032b}) as a  i8 is: {1:>4} ({1:>08b})", 128, 128 as i8);
    println!("1000 ({0:>032b}) as a  u8 is: {1:>4} ({1:>08b})", 1000, 1000 as u8);
    println!(" 232 ({0:>032b}) as a  i8 is: {1:>4} ({1:>08b})", 232, 232 as i8);
}