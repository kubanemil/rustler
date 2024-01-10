fn main() {
    let mut init = 0;
    loop {
        // basically loop == while true
        if init == 250 {
            break;
        }
        init += 1;
    }
    println!("init={}", init);

    let num = {
        loop {
            if init == 255 {
                break init * 2; // this how you return with loop
            }
            init += 1;
        }
    };
    println!("num={}", num);

    // loop labeling
    let mut i = 0;
    'outer1: loop {
        let mut j = 0;
        loop {
            j += 1;
            print!("{j} ");
            if j == 5 {
                break; // break always disrupts the inner loop
            }
            if i == 2 {
                break 'outer1; // break outer loop
            }
        }
        i += 1;
        println!("Inner loop over i={i}");
    }
    println!();

    // while loop
    let mut w = 0;
    while w < 5 {
        // nothing new to say
        w += 1;
        print!("w={w}{w} ");
    }
    println!();

    // for loop
    let arr = [12; 5];
    let mut sum = 0;
    for num in arr {
        // iterate over array
        sum += num;
        print!("{num} ");
    }
    println!("sum {sum}");

    // range for loop
    for i in 0..5 {
        // 0 to 4
        print!("{i} -> ");
    }
    println!();

    // reverse for loop
    for i in (0..5).rev() {
        // 4 to 0
        print!("{i}... ");
    }
    println!();
}
