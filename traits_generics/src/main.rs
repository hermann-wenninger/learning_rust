// traits and generics for structs enums tupels and functions
// struct Foo <T>{
//    field : T,
//                }
// enum Fool <T>{
//    Variante (T),
//  
//            }
// fn function<T>(val: T){}
//
//   struct Flusn <T>(T) ;   
//
//  impl<T> Floss<T> {}  

 enum Result <T, E> {
        Some(T),
        Error(E),
    }

struct Option<T>{
    Value: T,
}
impl<T> Option<T>{
    fn ok_or_error<E>(self error: E)->Result<T, E>{
        match self{
            Some(t) => Ok(t),
            None   => Err(err),
        }
    }
}
let size = size_of::<i32>();

fn parse<T>(s:&str)-> Result<T,()>{
    unimplementet!();
}
let x = "42".parse::<i32>();
let x ="42".parse().unwrap();
let x:Result<i32,_> = "322".parse();

fn makeit(bar:i32)->Result<i32,_>{
    let x ="42".parse();
    match x{
        Ok(ref mut x) => x + bar,
        None => {},
    }
fn min<T>(a:T, b:T)->T{
    where T: PartialOrd,
    {
        if a<b{a} else{b}
    }
}
//ODER
fn min<T:PartialOrd>(a:T, b:T){}
}
trait Goon{
 fn go_on(&self);
}
struct Car{name:String};
impl Goon for Car{
    fn go_on(&self){
        println!("wieder ein schritt weiter")
    }
}
let vw = Car{name:"volkswagen".into()};
vw.go_on();