use std::fmt::{Debug, Display};

/*
Trait is an interface (a collection of methods you can use on data type) */
trait Rating {
    fn popularity(&self) -> f32;
    fn is_good(&self) -> bool {
        // default implementation if not alternative given
        // you can use other methods even if they have no default implementation
        self.popularity() > 1 as f32
    }
}

#[derive(Debug)]
struct Manga {
    author: String,
    _chapters: u32,
    readers: i32,
}

// you need to implement all Rating's methods for Manga
impl Rating for Manga {
    fn popularity(&self) -> f32 {
        let popularity = self.readers as f32 / 1000.0;
        println!("{}'s popularity: {}", self.author, popularity);
        popularity
    }
}

#[derive(Debug)]
struct Book {
    author: String,
    is_classic: bool,
    number_of_copies: u32,
}

impl Rating for Book {
    fn is_good(&self) -> bool {
        let is_good = self.is_classic && self.number_of_copies > 1000;
        println!("{} is good: {}", self.author, is_good);
        is_good
    }
    fn popularity(&self) -> f32 {
        let popularity = self.number_of_copies as f32 / 1000 as f32;
        println!("{}'s popularity: {}", self.author, popularity);
        popularity
    }
}

// you can specify generic type with a trait (`T` is of type `Rating`)
fn is_bestseller<T: Rating>(content: &T) -> bool {
    content.is_good() && content.popularity() > 1.0
}

// you can add more traits:
fn _is_displayed_bestseller<T: Rating + Display>(content: &T) -> bool {
    content.is_good() && content.popularity() > 1.0
}

// also we can specify traits for several generics:
fn _some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    // do something
    0
}
// but we often write traits for generic types with `where` clause:
fn _some_function2<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // do something
    42
}

fn main() {
    let b1 = Book {
        author: "Dostoevsky".to_string(),
        is_classic: true,
        number_of_copies: 10000,
    };
    b1.is_good();
    b1.popularity();

    let m1 = Manga {
        author: "Isayama".to_string(),
        _chapters: 122,
        readers: 10000000,
    };
    m1.is_good();
    m1.popularity();

    let m2 = Manga {
        author: "ONE".to_string(),
        _chapters: 12,
        readers: 100,
    };
    println!("m1 is bestseller: {}", is_bestseller(&m2));
}
