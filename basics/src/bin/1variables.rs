fn main() {
    // by default you can't change variables after you define them
    let age = 19;
    println!("age={}", age);
    // age += 1; // trows an error

    let new_age = age + 1;
    println!("new_age={}", new_age); // but u can use it for new variables
                                     // also you can assign it to a new mutable variable
    let mut mut_age = age;
    mut_age *= 10;
    println!("mut_age={}; age={}\n", mut_age, age);

    // you can't use 'mut' with constants. Always specify the type and use uppercase for const
    const WW2_END_YEARS: u16 = 1945;
    println!("WW2_END_YEARS={}\n", WW2_END_YEARS);

    // scopes same as in python - variables defined in scope are not visible outside.
    // however, you can change outside varibles in scope (if mutable):
    let x = 10;
    let mut y = 20;
    {
        y *= 10;
        println!("1. inscope x={}, y={}", x, y);
        let (x, y) = (12, 22);
        println!("2. inscope x={}, y={}", x, y);
    }
    println!("outscope x={}, y={}\n", x, y);

    // .len()
    let s = "hello";
    println!("'{}'.len() = {}\n", s, s.len());
}
