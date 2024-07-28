use std::ops::Deref;

pub fn main() {
    let x = 12;
    let y = &x;
    let z = Box::new(x);

    assert!(x == *y);
    // you can use dereferencing with Box<>
    assert!(x == *z); // actually uses `*(z.deref())`

    let a = String::from("EMILIO");
    let b = MyBox::new(a.clone()); // or a will be consumed
    assert_eq!(a, *b);

    let c: &String = &b; // MyBox::deref() allows to convert to &String
    let d: &str = &b; // String::deref() allows to convert o &str
    assert_eq!(c, d);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // return reference, b.c. compiler can derefernce only `&` references
    } // if we returned value directly, it would be dropped from struct
}
