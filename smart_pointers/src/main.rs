use std::{collections::HashMap, env};
mod box_pointer;
mod deref_op;
mod drop_op;
mod rc_pointer;
mod ref_cell;

fn main() {
    let mut programs: HashMap<&str, fn()> = HashMap::new();
    programs.insert("box_pointer", box_pointer::main);
    programs.insert("deref_op", deref_op::main);
    programs.insert("drop_op", drop_op::main);
    programs.insert("rc_pointer", rc_pointer::main);
    programs.insert("ref_cell", ref_cell::main);

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
