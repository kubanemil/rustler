// this how you define module - it may contain anything you want.
// if you want something to work outside the module, use pub. 
pub mod rocket_tools { 
    mod components {
        pub fn build_rocket(mass: u32) -> Rocket {
            build_engine();
            build_tanks(); // can use, because siblings

            let mut rocket = Rocket::new(50, 500, 12);
            rocket.mass = mass;
            rocket
        }
        fn build_engine(){}
        
        fn build_tanks(){}
        
        pub struct Rocket {  // can use if have access to components
            pub width: u16,
            pub height: u32,
            mass: u32
        }

        impl Rocket {
            pub fn new(width: u16, height: u32, mass: u32) -> Self{
                Self {
                    width, height, mass
                }
            }
        }
    }

    pub mod performance {
        // think about `use` like this: 
        // let components = super::components;
        use super::components; // super is like .. in dir paths (parent)

        pub fn max_height() -> u32 {

            // components::build_engine(); // can't use because its private

            let mut rocket = components::build_rocket(120);
            // rocket.mass = mass // although we can't access mass, we can define it 
            // in ::build_rocket(mass), because that method has access for mass.
            let (height, _) = get_params(&rocket);
            height
        }

        fn get_params(rocket: &components::Rocket) -> (u32, u16) {
            let (h, w) = (rocket.height, rocket.width);
            // let m = rocket.mass // mass is private attribute
            (h, w)
        }
        
    }
}


/* You can access rocket_tools module here, even though it's private, because
`fn test_rocket()` and `mod rocket_tools` are siblings. */
pub fn test_rocket() {
    // use rocket_tools::performance::max_height; // relative path
    use crate::rocket_tools::{
        performance, // absolute path
        // components::Rocket, // isn't accessible (components is private, so blocks public Rocket)
    };

    /* can't use build_rocket(), because components is private 
    and not a sibling of 'fn test_rocket()' (even though build_rocket 
    by itself is public, you need to access ::components to use it) */

    // use crate::rocket_tools::components::build_rocket; 
    
    performance::max_height();
}

pub fn print_rocket() {
    for i in (1..3).rev() {
        println!("ROCKET{}!", i)
    }
}