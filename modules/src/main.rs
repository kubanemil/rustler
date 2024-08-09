// when using modules from lib.rs, no need to add 'mod lib'. It will rename lib -> project_name
use modules::{
    // module1 // is private
    print_rocket, // we don't specify that its from module2, because we used 'pub use'
    // use the project name, because project IS the module
    rocket_tools::{self, performance as rocket_performance}, // can give custom name to used module
};

/* if a module not from lib.rs, then you need to specify to which folder/file
to look for with: 'mod module_name;'. You need to do that only once*/
mod other_modules; // from other_modules.rs file
use other_modules::module1::{
    self,      // same as 'use other_modules::module1'
    module_fn, // same as 'use other_modules::module1::module_fn'
};

use std::collections::*; // imports all public modules from `std::collections`

/* convention is to specify module of function, but not structs:
use lib1::module1;
module1:fn1();

use lib1::module1::Struct1;
Struct1::new();*/
fn main() {
    other_modules::do_something();
    rocket_performance::max_height();

    // although it seems that we imported only perfromance module, by that
    // we also imported rocket_tools
    let h = rocket_tools::performance::max_height(); // same as above
    println!("Max height of rocket flight is {h}");
    println!();
    HashMap::new().insert('1', h);

    other_modules::subs_fn_print();
    print_rocket();
    module_fn();
}
