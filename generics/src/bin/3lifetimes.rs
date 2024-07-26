/*
We are not changing life time of x and y - we just specify that the output will live 
as long as both x and y will live. If some of parameter will die, then you gonna 
break the promise. */
fn longest<'a, T>(x: &'a[T], y: &'a[T]) -> &'a [T] {
    if y.len() >= x.len() {
        y
    } else {
        x
    }
}

// promise is that 'part' string will live at least as long
// as an instance of 'Whole' will.
struct Whole<'a> {
    part: &'a str
}

fn promise_violation() {
    let x = ["abc", "ker"];
    // let result;
    // {
    //     let y = ["be", "movie", "donio"];
    //     // funct will return reference to y (bc its longer), so result = &y, and y dies,
    //     // so result is dangling pointer
    //     result = longest(&x, &y); 
    // }
    // println!("the result: {:?}", result);
}

// // cant return reference to res, because res will be deleted at the end of function
// fn self_reference(x: &str, y: &str) -> &str {
//     let res = String::from("Emilio Kubani");
//     &res
// } 

fn main() {
    let l1 = [1, 2, 3];
    let l2 = [7; 5];
    let longest_l = longest(&l1, &l2);
    println!("{:?}", longest_l);
}

fn dangling_pointers() {
    let r;
    {
        let x = 10;
        r = &x; // problem is that value dies earlier than its reference
    } // x dies here 
    // println!("{}", r); // and creates dangling reference
}
/*
3 Rules of Life-Time:
1. Each parameter gets it's own lifetime
2. If only one input, then:  
    let `lifetime of output reference` = `lifetime of input reference`
3. If one of parameters is &self, then:
    let `lifetime of output reference` == `lifetime of &self` */