fn ein_tupel(str:String)->(String, usize){
    let laenge = str.len();
    (str, laenge)
}
fn ein_tupel_von_ref(str:&String)->(String, usize){
    //let laenge = str.len();
    (str.to_string(), str.len())
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
    let cloner =  ein_tupel(wanderer.clone());
    println!("{} ---> und habe die länge {} cloning",cloner.0, cloner.1);
    let borr = ein_tupel_von_ref(&wanderer);
    println!("{} ---> und habe die länge {} borrowing",borr.0, borr.1);

    //verändern einer variablen
    let mut erneuerer = String::from("Heute arbeiten wir an:");
    fn newbee(str: &mut String){
        str.push_str(" veränderbaren referenzen");
    }
    newbee(&mut erneuerer);
    println!("{:?}",erneuerer);

    fn nicht_hängend () -> String {
        let s = String::from("Hallo ich hänge nicht!");
    
        s
    }
    let oo = nicht_hängend();
    println!("{}",oo);
}
