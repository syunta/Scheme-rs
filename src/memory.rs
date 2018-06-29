use data::Obj;
use std::cell::RefCell;
use std::collections::HashMap;

static mut FREE_PTR: i32 = 0;

thread_local!(pub static WORKING_MEMORY: RefCell<Memory> = {
    let mut mem = Memory { cars: HashMap::new(), cdrs: HashMap::new() };
    RefCell::new(mem)
});

pub struct Memory {
    cars: HashMap<i32, Obj>,
    cdrs: HashMap<i32, Obj>,
}

fn cons(car: Obj, cdr: Obj) -> Obj {
    WORKING_MEMORY.with(|mem| {
        mem.borrow_mut().cars.insert(FREE_PTR, car);
    });
    Obj::Nil
}