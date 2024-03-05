fn ein_tupel(str:String)->(String, usize){
    let laenge = str.len();
    (str, laenge)
}

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
fn nimmt_und_gibt_zurück(ein_str: String) -> String { // a_string kommt in den
    ein_str  
}
fn main() {
    let zu_und_zurück = String::from("ich wandere von einem zum andern");
    let s = String::from("ich lande auf dem heap");
    let x = 21;
    let literal = "literal".to_string();
    let von_funk = gib_ownership();
    println!("von funktion ------>{}",von_funk);
    take_literal(x);
    println!("und danach lande ich nochmal---->{}",x);
    take_literalx(&literal);
    println!("nochmal--->{literal}");
    let wanderer = nimmt_und_gibt_zurück(zu_und_zurück);
    println!("ich bin ein makro und habe die eigentümerschaft von: {}",wanderer);
    println!("ich bin ein makro und habe die eigentümerschaft von: {}",wanderer);
    println!("ich bin ein makro und habe die eigentümerschaft von: {}",wanderer);
    let pull =  ein_tupel(wanderer);
    println!("{} ---> und habe die länge {}",pull.0, pull.1);
}
