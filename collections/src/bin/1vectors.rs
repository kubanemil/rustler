/*
Vector like a String, has no fixed size and stored in a heap.
You should also always reference them, because no built-in copy. */

fn main() {
    let mut ages: Vec<i32> = Vec::new(); // should annotate, bc can't infere type of empty vector.
    let mut v = vec![1, 2, 3, 4, 5]; // can create with `vec!()` macro
    for num in &v {
        print!("{num} ");
    }
    v.push(12); // .push() -> append | .append() -> extend
    println!("\nv: {:?}", v);

    for i in 18..28 {
        ages.push(i)
    }
    println!("ages: {:?}", ages);

    let last_age = ages.last().unwrap();
    // .get() won't return error, so gives Option<T> (None, if index is out of bound)
    let optional_last_age = ages.get(ages.len() + 100);
    if let Some(age) = optional_last_age {
        println!("The last age is {age}");
    } else {
        println!("optional_last_age is null, becuse index is out of bound!");
    }

    // ages.push(120); // can't change until 'last_age' is borrowing from 'ages'
    println!("last_age is {last_age}"); // still borrowing here

    // you can change each element in vector:
    for age in &mut ages {
        // denote that its a mutable reference
        *age += 100; // *age - is dereference
    }
    println!("mutated ages: {:?}", ages);

    // this won't work, because vector requires the same type for each element:
    // let ifs_vector = vec![112, "Emil", 0.4221];

    use Ifs::{F, I, S};
    // but you can bypass this restrictions with enum:
    let ifs_vector: Vec<Ifs> = vec![F(0.1231), I(32), S("EMIL".to_string()), I(199)];
    println!("ifs_vector: {:?}", &ifs_vector);
    let value = match &ifs_vector[0] {
        I(int) => int,
        S(name) => &name.clone().parse::<i32>().unwrap(),
        F(float) => &(float.round() as i32),
    };
    println!("{value}");
}

#[derive(Debug, Clone)]
enum Ifs {
    I(i32),
    S(String),
    F(f64),
}
