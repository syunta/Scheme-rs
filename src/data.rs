use std::fmt;
use std::rc::Rc;

pub enum Obj {
    Nil,
    Int(i32),
    Cell(Rc<Obj>, Rc<Obj>)
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_string(self))
    }
}

fn format_string(obj: &Obj) -> String {
    match *obj {
        Obj::Nil => "()".to_string(),
        Obj::Int(i) => i.to_string(),
        Obj::Cell(ref car, ref cdr) => {
            "(".to_string()
                + &car.to_string()
                + &format_cdr(cdr)
                + ")"
        }
    }
}

fn format_cdr(obj: &Obj) -> String {
    match *obj {
        Obj::Nil => "".to_string(),
        Obj::Cell(ref car, ref cdr) => {
            " ".to_string() + &car.to_string() + &format_cdr(cdr)
        },
        ref atom => " . ".to_string() + &atom.to_string()
    }
}

#[test]
fn it_works() {
    let a = Obj::Cell(Rc::new(Obj::Int(10)), Rc::new(Obj::Nil));
    let b = Obj::Cell(Rc::new(Obj::Int(20)), Rc::new(a));
    let c = Obj::Cell(Rc::new(Obj::Int(30)), Rc::new(b));
    assert_eq!("(30 20 10)", format_string(&c));

    let d = Obj::Cell(Rc::new(Obj::Int(1)), Rc::new(Obj::Int(2)));
    assert_eq!("(1 . 2)", format_string(&d));

    let e = Obj::Cell(Rc::new(c), Rc::new(d));
    assert_eq!("((30 20 10) 1 . 2)", format_string(&e));
}
