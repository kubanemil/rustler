pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test] // indicates that this is a test function
    fn add_str() {
        let mut s = String::from("Emil");
        s.push_str(" is cool!");
        assert_eq!(s, "Emil is cool!"); // prints left and right value if error
        assert_ne!(s, "not s for sure"); // should pass ok if left and right are not equal
    }

    #[test]
    fn pow2() {
        let mut my_num = 12;
        let power = 2;
        for _ in 1..power {
            my_num *= my_num
        }
        // panic!("OH MY GOD!!!");
        assert!(!(my_num == 12 + 12)); // !true -> false
        assert!((my_num == 12 * 12));
        assert!({
            let to_compare = 12 * 12;
            my_num == to_compare
        }); // same as above

        // arguments after first one is basically print!()'s arguments:
        assert!(
            (my_num == 12 * 12),
            "Duuuuude, `my_num` is not 12*11, you know?!: {}",
            my_num
        );
    }
}

pub struct Adult {
    age: i32,
}

impl Adult {
    pub fn new(age: i32) -> Adult {
        if age < 18 {
            panic!("You are not adult! {}-years old are not allowed.", age);
        } else if age > 100 {
            panic!(
                "Maan, normal people don't live up to {} years, you know...",
                age
            );
        }

        let adult = Adult { age };
        println!("Created age: {}", adult.age);
        adult
    }
}

// Expected panics
#[cfg(test)]
mod panic_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn oldman() {
        Adult::new(200);
    }

    #[test]
    #[should_panic(expected = "jojo is good")]
    fn adult() {
        Adult::new(35);
        //panic!("jojo is the worst anime."); // will panic bc unexpected
        panic!("you know, jojo is good!"); // won't panic bc expected
    }

    #[test]
    #[should_panic(expected = "You are not adult!")]
    fn kid() {
        Adult::new(11);
    }
}

// Result<T, E> testing

#[cfg(test)]
mod result_tests {
    use std::{fs::File, io::Error as IoError};

    #[test]
    fn it_works_ad_() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            // if Err is returned, than test will fail
            Err(String::from("two plus two does not equal five"))
        }
    }

    #[test]
    fn open_absent_file() -> Result<(), IoError> {
        File::open("absent.txt")?;
        Ok(())
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!("Lol".len(), 3);
    }
}

/*
by default all tests are run parallel. If you want to run it sequential:
$ cargo test -- --test-threads=1    // don't forget double --

also testing doesn't output your println!() stuff, to output them:
$ cargo test -- --show-output

if you want to run specific test case:
$ cargo test open_absent_file    // specify test fn's name
Note, that it will run all test with name 'fn open_absent_file()'


if you want to run tests that contain some string:
$ cargo test ad
In this case, will run tests 'fn adult()', 'fn it_works_ad_()', and 'fn add_str()'

if you want to run tests marked with '#[ignored]':
$ cargo test -- --ignored    // will run only ignored tests
$ cargot test -- --include-ignored  // will run all tests
*/
