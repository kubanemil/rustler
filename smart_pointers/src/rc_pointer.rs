/* You will use Rc<T> (Reference Counted Smart Pointer) when you need to have
multiple owners for a value.
*/

use std::rc::Rc;

pub fn main() {
    use List::{Cons, Nil};

    let x = Rc::new(Cons(11, Rc::new(Cons(13, Rc::new(Nil)))));
    println!("count new = {}", Rc::strong_count(&x));

    // Rc::clone() increases count and creates additional pointer to the pointer

    // `Rc::clone(&x)` == `x.clone()`, but we want to distinguish that Rc::clone() is shallow-copy
    // (it will only copy pointer and increase pointer), while in most cases .clone() is deep-copy
    let _a = Cons(1, Rc::clone(&x));
    println!("count after creating a = {}", Rc::strong_count(&x));
    let _b = Cons(2, Rc::clone(&x));
    println!("count after creating b = {}", Rc::strong_count(&x));
    {
        let _c = Cons(4, Rc::clone(&x));
        println!("count after creating c = {}", Rc::strong_count(&x));
    } // count will go down since c is out of scope
    println!("count final = {}", Rc::strong_count(&x));
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
