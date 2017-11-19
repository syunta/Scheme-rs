use std::fmt;

pub enum Obj {
    Nil,
    Int(i32)
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Obj::Nil => write!(f, "{}", "()"),
            Obj::Int(i) => write!(f, "{}", i)
        }
    }
}
