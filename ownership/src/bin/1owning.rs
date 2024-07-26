/*
Stack - is queue that stores fixed-size data
Fast retrieve/insert.
Heap is for dynamic-size data
Slower insert/retrieve process (need to follow pointer)
Pointer to heap is in stack, because it's fixed-size

Ownership rules:
1. Each value has only one owner.
2. If owner goes to outer scope, value is dropped (from memory). 
*/
fn main() {
    let mut s = "hello"; // string literal (&str) can't be mutable
    //s.push_str(", world"); // error
    let mut s1 = String::from(s); // String is mutable and dynamic-sized
    s1.push_str(", world");
    println!("|{s}| |{s1}|");

    let name = {
        let k = String::from("Kolya");
        k
    }; // k is dropped here
    println!("{}", name);

    /*
    When you copy a primitive type (bool, int, float), Rust just copies it 
    while assigning to the variable,
    because they are of fixed size and stored in a stack.
    But you can't do this with more complex types (String, Vec), because their
    size is limitless, which theoretically means infinite size (something like 1TB). 
    And we don't want to copy such a huge variable.

    So what Rust does instead, is just gives a pointer from one variable to another.
    Also, we cannot point 2 pointers to one heap value, because it will create a memory problems. 
    (like if you change one variable, you are changing the second one too)
     */
    let s1 = String::from("Emil info");
    let s2 = s1; // s1 is moved to s2. We call this "MOVE"
    /*
    After that, s1 is dropped, so you can't access it anymore. But if you want to copy value
    (so both s1 and s2 will have copies of value), do .clone() method:
     */
    let s3 = s2.clone();
    println!("[s1 is null| |s2 is {}| |s3 is {}|\n", s2, s3);

    let s1 = String::from("Take this string");
    // take_str() => let s = s1 <- assigns s1 to function variable s
    take_str(s1);
    // s1 is not available anymore!
    //println!("{}", take_str_var); // error

    scope_memory();
}

/* Be careful with scopes, because it drops all dynamic-sized arguments
at the end of the scope, instead of borrowing them
(You can't use your given variables after the funtion) */
fn take_str(s: String) {
    dbg!(s);
} // s is dropped here


// How scope memory works
fn scope_memory() {
    let s1 = String::from("five");
    let int1 = 5;
    let f1 = 5.0;
    let b1 = true;
    println!("{} {}, {}, {}", s1, int1, f1, b1);
} // drop(s1, int1, f1, b1) - drops all variables here