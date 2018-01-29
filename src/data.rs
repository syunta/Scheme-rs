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
