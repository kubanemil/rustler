fn main() {
    let name = String::from("Emil");
    let result = use_and_drop(name);
    // println!("result='{result}'; name='{name}'"); // error, because 'name' is dropped

    // add reference to borrow (&name or & name):
    let mut name = String::from("Emil Prime");
    let result = borrow_and_use(& name); // '&' is a reference to a variable
    // Note, that &String (0x1231235, String) is not String ("Hello World")
    println!("result='{result}'; name='{name}';");

    borrow_and_change(&mut name); // also specify that reference is mutable when giving

    let ref_name = &name;
    // You can borrow same variable twice as immutable ()
    let second_ref_name = &name;
    // but you can't have 2 mutable references (or 1 mutable/1 immutable) at the same time
    // let third_ref_name = &mut name;' // error
    println!("ref_name='{}' | second_ref_name='{}'", ref_name, second_ref_name);
    let third_ref_name = &mut name; // possible because ref_name is used
    println!("{third_ref_name}");
    // ref_name.push_str("!!!")  // but if you will add this,
    // then there is two different references to name, so it will be an error

    /* 
    Explaination for above code:
    the variable will be dropped if:
     1. It goes out of introduced scope
     2. At the last time it is used.
    In above code, although 'ref_name' is still in scope, it was dropped when we
    used it last time, so creating third reference is possible, because it is actually
    the only reference to name (ref_name and second_ref_name are already dropped). 
    Logically, the last usage seen in compile time, not runtime.
    So if there is any usage of ref_name after third_ref_name, an error will be thrown.
     */
}

fn use_and_drop(s: String) -> String {
    println!("I used your variable '{s}', lol!"); println!();
    let dummy = String::from("lol");
    dummy
}

// when you borrow, you always need to give variable back (no dropping)
fn borrow_and_use(s: &String) -> String {
    println!("I borrowed your variable '{s}', sir."); println!();
    let dummy = String::from("lol");
    dummy
}

// even if 's' is mutable, you need to specify that reference
// is also mutable (by '&mut') to change it:
fn borrow_and_change(s: &mut String) {
    s.push_str(" is cool!"); 
    println!("I changed your variable to '{s}', sir."); println!();
    
}

/* Rust ensures that there will never be dangling reference 
(a pointer to the data that doesn't exist anymore) */
fn dangling_reference() {
    let s = String::from("JoJo");
    // return &s // should be 'return s'
} // s is dropped here, but reference is still there, so it is an error