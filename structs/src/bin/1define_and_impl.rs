/*
Struct
 collection of object's attribute
 */

#[derive(Debug)] // so we can use {:?} in println!()
struct Person {
    // can't use &str (without lifetime), 
    // bc if u drop str, then name is dangling pointer
    // also Struct should own its fields' data, not borrow
    name: String, 
    age: u8,
    height: i16,
}

fn main() {
    let user1 = Person::create(
        "Koichi",
        15,
        -15,
    );
    user1.present();

    // if variable has same name as attribute, u can use shorthand:
    let name = "Jotaro".to_string();
    let height = 190;
    let user2 = Person {
        name, // shorthand
        age: 22,
        height, // shorthand
    };
    user2.present();

    // you can create new object with some attribute from another object:
    let mut user3 = Person {
        name: "Kakyoin".to_string(),
        ..user2 // copy all attribute (except name) from user2
    };
    user3.height = 176; // entire instance should be mutable
    user3.present();

    let green = RGB(255, 0, 0);
    green.present();

    let unit = Unit;

    println!("{:#?}", user3); // '#' - print with indent ':?' - debug print
    // also you can print struct with dbg!() macro:
    dbg!(user2); // you still need to add derive(Debug) to struct
    let mut mob = Person::create_mob();
    mob.name.push_str(" is A GOAT!");
    let mob = dbg!(mob); // (returns what u give)
}

// tuple Struct
struct RGB(u8, u8, u8);

// unit Struct
struct Unit;

// impl - implementation. The collection of associated functions.
impl Person {
    // if includes &self - method
    fn present(&self) {
        println!("{} is {} years old and {} cm tall", self.name, self.age, self.height);
    }

    // not method. call with `Person::create(name, age, height);`
    fn create(name: &str, age: u8, height: i16) -> Self {
        Self {
            name: name.to_string(),
            age,
            height,
        }
    }
}
impl Person { // can have multiple impl for same struct. No any difference at all.
    // Person::create_mob();
    fn create_mob() -> Self {
        Self {
            name: "Mob".to_string(),
            age: 14,
            height: 160,
        }
    }
}

impl RGB {
    fn present(&self) {
        println!("RGB({}, {}, {})", self.0, self.1, self.2);
    }
}