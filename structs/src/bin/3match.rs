enum SorcererClass {
    _Third(String),
    _Second,
    First,
    Special(String),
}

fn get_power(class: SorcererClass) -> f64 {
    // match should handle all options
    match class {
        // matches with first match (even if there are several possible matches)
        SorcererClass::First => 0.5, // arm
        SorcererClass::Special(name) => {
            // arm (can be scope or function if returns right type)
            if name == "Gojo" {
                println!("Its our boy Gojo!");
                let gojos_power = 10.0 * 100.0;
                gojos_power * 1000.0
            } else {
                1.0
            }
        }
        _weaklings => 0.1, // you can call the remaining matches as you want
                           // but we usually use:
                           // _ => 0.1;
    }
}

fn main() {
    let nanami_class = SorcererClass::First;
    let nanami_power = get_power(nanami_class);
    println!("Nanami's power: {nanami_power}");
    get_power(SorcererClass::Special("Gojo".to_string()));

    /* use 'if let' as shorthand of match */
    let geto = Some("Suguru Geto");
    if let Some(name) = geto {
        // Will match with any Some that stores string
        println!("{name} is here!");
    }
    /*
    same as this:
    match geto {
        Some(name) => println!("By match: {name} is here!"),
        _ => {},
    }
     */

    // you can also add 'else' for _:
    let special_one = SorcererClass::Special("Sukuna".to_string());
    if let SorcererClass::_Third(name) = special_one {
        println!("The name of weakling is {name}")
    } else if let SorcererClass::Special(name) = special_one {
        println!("The name of special: {name}")
    } else {
        println!("It is second or first class sorcerer")
    }
    // usually need 'if let' to extract value from some enum/struct
}
