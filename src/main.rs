mod data;
mod parser;
use std::rc::Rc;

fn main() {
    let hoge = data::Obj::Cell(Rc::new(data::Obj::Int(10)), Rc::new(data::Obj::Nil));
    let huga = data::Obj::Cell(Rc::new(data::Obj::Int(20)), Rc::new(hoge));
    let piyo = data::Obj::Cell(Rc::new(data::Obj::Int(30)), Rc::new(huga));
    println!("{}", piyo);

    let mut p = parser::Parser { unparsed: "10".to_string() };
    p.parse();
}
