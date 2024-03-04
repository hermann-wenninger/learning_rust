fn gib_ownership()->String{
    let ein_string = String::from("ich komme aus einer funktion");
    ein_string
}
fn take_literal(lit:u8){
    println!("i am in a funktion --->{}",lit)
}
fn take_literalx(litx:&String){
    println!("i am in a funktion --->{}",litx)
}
fn main() {
    let s = String::from("ich lande auf dem heap");
    let x = 21;
    let literal = "literal".to_string();
    let von_funk = gib_ownership();
    println!("von funktion ------>{}",von_funk);
    take_literal(x);
    println!("und danach lande ich nochmal---->{}",x);
    take_literalx(&literal);
    println!("nochmal--->{literal}")
}
