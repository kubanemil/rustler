use std::{collections::HashMap, env};
mod box_pointer;
mod deref_op;
mod drop_op;
mod rc_pointer;


fn main() {
    let mut programs: HashMap<&str, fn()> = HashMap::new();
    programs.insert("box", box_pointer::main);
    programs.insert("deref", deref_op::main);
    programs.insert("drop", drop_op::main);
    programs.insert("rc", rc_pointer::main);

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.get(1) {
        None => {
            println!("Are you stupid, bro? Provide function to run in CLI");
        }
        Some(program_name) => {
            let program = programs.get(program_name.as_str());
            match program {
                None => {
                    println!("Sorry bro, no such program(")
                }
                Some(func) => {
                    func();
                    println!("\nFinished {} program", program_name);
                }
            }
        }
    }
}
