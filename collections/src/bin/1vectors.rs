/*
Vector like a String, has no fixed size and stored in a heap.
You also should alway reference them, because no built-in copy. */

fn main() {
    let mut ages: Vec<i32> = Vec::new(); // should annotate, bc can't infere type of emtpy vector.
    let mut v = vec![1,2,3,4,5]; // can create with `vec!()` macro
    for num in &v {
        print!("{num} ");
    }
    v.push(12); // .push() -> append | .append() -> extend
    println!("\nv: {:?}", v);

    for i in 18..28 {
        ages.push(i)
    }
    println!("ages: {:?}", ages);

    let last_age = &ages[ages.len()-1];
    // .get() won't return error, so gives Option<T> if index is out of bound
    let optional_last_age = ages.get(ages.len()+100);
    if let Some(age) = optional_last_age {
        println!("The last age is {age}");
    } else {
        println!("optional_last_age is null, becuse index is out of bound!");
    }

    // ages.push(120); // can't change until 'last_age' is borrowing from 'ages'
    println!("last_age is {last_age}"); // still borrowing here

    // you can change each element in vector:
    for age in &mut ages { // denote that its a mutable reference, not simple refrence
        *age = *age + 100; // *age - is dereference
        // *age += 100; // same as above
    }
    println!("mutated ages: {:?}", ages);

    // this won't work, because vector requires the same type for each element:
    // let ifs_vector = vec![112, "Emil", 0.4221];

    use IFS::{I, S, F};
    // but you can bypass this restrictions with enum:
    let ifs_vector: Vec<IFS> = vec![
        F(0.1231),
        I(32), 
        S("EMIL".to_string()), 
        I(199), 
    ]; println!("ifs_vector: {:?}", ifs_vector);
}

#[derive(Debug)]
enum IFS {
    I(i32), S(String), F(f64)
}