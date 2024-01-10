fn get_next(n: i32) -> i32 {
    // you always need to specify argument's type
    println!("Next value is {}\n", n + 1);
    n + 1 // return value
}
fn main() {
    let n = 12;
    get_next(n);

    // expressions
    let x = 5; // statement (doesn't return anything)
    let y = {
        // expression (bc has return value)
        let x = 3;
        x + 1 // return value (don't but ';' sign)
    };
    println!("x={}", x);
    println!("y={}\n", y);

    let num = five();
    println!("num={}\n", num);

    let lucky = is_lucky(7);
    println!("lucky={}", lucky);
    // inline if statement
    let result = if lucky { "you won!" } else { "you lost!" }; // structure is same (unlike python)
    println!("result={}", result);
    // returns should be of same type
    // let result2 = if lucky {10000} else {"2000"}; // throws error
}

fn five() -> i32 {
    5 // same as 'return 5;'
}

fn is_lucky(num1: i32) -> bool {
    // always pass bool into if statement
    // if num1 {}   // throws error
    if num1 == 0 {
        // no need for (num1==0)!
        println!("Empty heart");
        false
    } else if num1 == 7 {
        // elif
        println!("Lucky number");
        true
    } else {
        println!("Unlucky number");
        false
    }
}
