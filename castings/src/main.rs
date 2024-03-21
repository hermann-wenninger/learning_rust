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

   
}