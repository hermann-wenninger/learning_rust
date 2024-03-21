fn main(){
let mut data = vec![1, 2, 3];
// get an internal reference
let x = &data[0];
println!("{}", x);


// ---> println!("{}", x) does not compile at this place

data.push(4);
println!("{:?}",data)


}