fn main() {
    // specify type annotation to parse
    let guess: i32 = "42".parse().expect("Not a number!");
    println!("guess={}\n", guess);

    let mut max_num: u8 = 250; // max is 2**8 - 1 = 255
                               // in debug mode will panic. In release mode it will be deasastrous.
                               // (256 -> 0, 257 -> 1, 258 -> 2, etc.)
    max_num += 8; // 250 + 8 = 258, but will see '2' in release mode
    println!("max_num={}\n", max_num);

    // floating point types
    let x = 2.0; // f64, but can specify f32 (all floats are signed)
    println!("x={}", x);

    // math operations. Same as in python, except:
    let result = 5 / 3;
    println!("5/3={}", result); // 1, not 1.6666666666666667
                                // to get float result, both nums should be floats (result's data specifcation won't help)
    let result = 5.0 / 3.0;
    println!("5.0/3.0={}", result); // 1.6666666666666667
                                    // remainder
    let result = 15 % 13;
    println!("15/13={}\n", result); // 2

    // char (only works for one character)
    let c: char = 'z';
    println!("c={}\n", c);

    // tuple
    let tup: (i32, f64, &str) = (500, 6.4, "hello");
    let tup1 = tup.0;
    println!("tup1={}", tup1);
    let (a, b, c) = tup;
    println!("(a, b,c)={}, {}, {}\n", a, b, c); // you can assign like that

    // array
    let arr: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0]; // [type; size]
    let threes = [3; 10];
    println!("threes={:?}", threes); // [3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
    let arr1 = arr[0];
    println!("arr1={}", arr1);
}
