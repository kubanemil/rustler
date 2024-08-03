/* Rust always makes you specify the data type, but sometimes
you don't care what type is, so you should use generics,
You usually append <T, U, K> to function/struct's name:
fn do_something<T> | struct Person<T>
And your <T> will represent general type */

// here we don't care about the data type - we will get the last item despite of it
fn last_item<TheType>(arr: &[TheType]) -> &TheType {
    // usually `T` is used, not `t`.
    let item = &arr[arr.len() - 1];
    item
}

// `<T: std::cmp::PartialOrd>` - TRAIT that allows comparable types only,
// otherwise compile error will be raised, because not all possible types are comparable
fn largest<T: std::cmp::PartialOrd>(arr: &[T]) -> &T {
    let mut largest = &arr[0];

    for item in arr {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    // although T might be any type, both x and y must be of same type
    _x: T,
    _y: T,
}
#[derive(Debug)]
struct Point2<T, U> {
    // if you want to x and y be of different types
    x: T,
    y: U,
    z: T,
}

// generic type specification after `impl` is neccessary clarification
impl<T, U> Point2<T, U> {
    fn _x(&self) -> &T {
        &self.x
    }
}
// if you want to define methods only for 'f32' data type for Point2 struct:
impl Point2<f32, f32> {
    fn ratio(&self) -> f32 {
        &self.x / &self.y * &self.z
    }
}

fn main() {
    let int_arr = vec![1, 2, 3, 4, 5];
    let str_arr = ["one", "two", "three"];
    let bool_arr = [true, true, false, false, true, false];
    println!(
        "last int, str, bool: ({}, {}, {})",
        last_item(&int_arr),
        last_item(&str_arr),
        last_item(&bool_arr)
    );

    println!(
        "largest int, str, bool: ({}, {}, {})",
        largest(&int_arr),
        largest(&str_arr),
        largest(&bool_arr)
    );

    let p1 = Point { _x: 21, _y: 93 };
    dbg!(&p1);
    let p2 = Point2 {
        x: 32,
        y: "Lol",
        z: 12,
    };
    dbg!(&p2);
    let p3 = Point2 {
        x: 10.0,
        y: 4.012,
        z: 0.129,
    };
    // p2.ratio(); // ratio is only for Point<f32, f32> type
    p3.ratio();
}
