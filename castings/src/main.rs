// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    
    let integer: u8 = decimal as u8;
   

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

   
    let character = integer as char;
   

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}