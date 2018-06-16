use data::Obj;

static mut FREE_PTR: i32 = 0;

struct Memory {
    cars: [Obj; 100],
    cdrs: [Obj; 100],
}