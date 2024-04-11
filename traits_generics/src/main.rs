fn main(){
let mut x = String::from(" das ist ein satz! ");
x.trim();
let xx = &x.as_bytes();
let mut xxxx = 0;
for (i, &xxx) in xx.iter().enumerate(){
    if xxx == b' '{
     let y = i;   
    println!("{}",&x[..i]);
}else{
    
    println!("{}","sdfgh".to_string())
}
let v = vec!("aloha", "samba","coltronw","silversee");
for i in &v{
    println!("{}",i);
}
for name in v.iter(){
    match name{
        &"silversee" => println!("asdfrgtzhu"),
        _ => println!("Hello {}", name),
    }
}
}}