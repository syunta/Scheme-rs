mod data;
use std::rc::Rc;

fn main() {
    let hoge = data::Obj::Cell(Rc::new(data::Obj::Int(20)), Rc::new(data::Obj::Nil));
    let huga = data::Obj::Cell(Rc::new(data::Obj::Int(20)), Rc::new(data::Obj::Int(100)));
    println!("{}", hoge);
    println!("{}", huga);
}
