fn greet_a(name: &String){
    println!("{}",name);
}

fn greet_b(name:&mut String){
    println!("{}",name);
    *name = "Hamann".to_string();
    name
}

fn greet_c(mut name: String){
    name = "hmann".to_string()
}

//different types
String
&String
&mut String
&str
*string //dref the binding

let x = "Arnika".to_string();
greet_b(&mut x);

greet_a(&x);

//aliasing or mutability//

//drop
fn drop(variable: Typ){}//is leer vuoto empty we loose the type the binding ALL!!!!