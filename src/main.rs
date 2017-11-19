mod data;

fn main() {
    let hoge = data::Obj::Nil;
    let huga = data::Obj::Int(100);
    println!("{}", hoge);
    println!("{}", huga);
}
