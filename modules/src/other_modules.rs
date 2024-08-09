pub fn do_something() {
    println!("doing something...")
}

pub mod module1 {
    pub fn module_fn() {
        super::do_something();
    }
}

use crate::module1::module_fn;

mod module2 {
    pub fn _fn1() {
        println!("crate::module2::module2_fn()");
        // module_fn(); // error, because module_fn defined outside of scope
    }

    pub fn _fn2() {
        use crate::module1::module_fn; // need to use in the scope you need
        module_fn();
    }

    pub mod submodule2 {
        pub mod subsubmodule2 {
            pub fn subs_fn_print() {
                println!("module2::submodule2::subsubmodule2::subs_fn_print()")
            }
        }
    }
}

fn _kk() {
    module_fn() // works because in scope
}

// now can skip `module2::submodule2::subsubmodule2::` when importing `subs_fn_print()`:
pub use module2::submodule2::subsubmodule2::subs_fn_print;
