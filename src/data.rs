use std::fmt;
use std::rc::Rc;

pub enum Obj {
    Nil,
    Int(i32),
    Cell(Rc<Obj>, Rc<Obj>)
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Obj::Nil => write!(f, "{}", "()"),
            Obj::Int(i) => write!(f, "{}", i),
            Obj::Cell(ref car, ref cdr) => {
                write!(f, "(").unwrap();
                write!(f, "{}", car).unwrap();
                print_cdr(cdr, f).unwrap();
                return write!(f, ")");
            }
        }
    }
}

fn print_cdr(obj: &Obj, f: &mut fmt::Formatter) -> fmt::Result {
    match *obj {
        Obj::Nil => write!(f, ""),
        Obj::Cell(ref car, ref cdr) => {
            write!(f, " {}", car).unwrap();
            print_cdr(cdr, f)
        },
        ref atom => write!(f, " . {}", atom)
    }
}
