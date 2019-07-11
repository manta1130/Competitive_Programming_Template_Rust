use std::ops;

#[derive(PartialEq, Eq)]
struct Modint<T> {
    p: isize, //modint
    v: T,
}
impl<T> Modint<T> {
    fn new(p: isize, v: T) -> Modint<T> {
        Modint { p, v }
    }
}

impl<T> Add for Modint<T> {
    fn add(self,other:Modint<T>) -> Modint<T>{
        if self.p != other.p{
            panic!("Mod Error");
        }
    }
}

fn main() {}
