/*
Box is just a pointer to the heap data

Use Box<T> if:
    - using a T that is not a fixed-size type, but you want to use T in a context that requires exact size
    - When you have large data (probably fixed-size), and don't want it to be copied when transfering
    ownership. (As you remember, fixed-size (stack) data is copied when changing owner, but heap data 
    will only change pointer).
    - when you want to own a data, and you care that it only has certain traits, rather than specific type
*/

pub fn main() {
    let num_in_heap = Box::from(5);
    let box_address: *const Box<i32> = &num_in_heap as *const Box<i32>;
    println!("box_address: {:p}", box_address);
    println!("box_reference: {:p}", num_in_heap);
    
    use List::{Cons, Nil};
    let newb = Box::new;

    let list = Cons(1, newb(Cons(2, newb(Cons(3, newb(Cons(4, newb(Nil))))))));
    println!("{:?}", list);
    let _type_ = match list {
        Cons(num, pointer) => {
            println!("{:?}", pointer);
            Some(num)
        },
        Nil => None
    };
} // when list is out of scope, it deletes not only Box<> pointers, but also a heap data.

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}