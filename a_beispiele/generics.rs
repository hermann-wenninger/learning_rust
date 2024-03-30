fn fooo<T>(arg:T){}

struct QA;

struct Just(QA);

struct JustGen<T>(T);

let _x = Just(QA);
let _ch:JustGen<char>= JustGen('A');

let _d = JustGen(QA);
let _z = JustGen(88);
let _p = JustGen('I');

fn regular_func(arg: Just){};
fn func_spez_i32(arg:JustGen<i32>){};
fn generic<T>(arg:JustGen<T>){};
fn func_spez_t(arg:JustGen<QA>){};

fn main(){
    generic::<char>(JustGen('a'));
    generic(JustGen('a'));
}
//implementatione
struct A;
struct GenericVal<T>(T);
impl GenericVal<i32>{};
impl GenericVal<A>{};
impl<T> GenericVal<T>{};

//concrete impl block
struct Generic_Value<T>{
    gen_val:T;
}
impl<T> GenericVal<T>{
    fn get(&self)-> &T{
        &self.gen_val
    }
}
let xyz = Generic_Value{gen_val:88i32};
//get referenz
let zappi = xyz.get();