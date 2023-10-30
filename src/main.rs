use std::env;

mod definitions {
    pub mod def;
}

mod threaded {
    pub mod mod_91;
    pub mod mod_100;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // If no arguments are provided, run the def() function
        definitions::def::run(); // Call the function from the def module
    } else {
        match args.len() {
            1 => run_threaded("100"), // cargo run
            2 => {
                if &args[1] == "threaded" {
                    run_threaded("100"); // cargo run threaded
                } else {
                    println!("Invalid arguments.");
                }
            }
            3 => {
                if &args[1] == "threaded" {
                    let mod_to_run = &args[2];
                    run_threaded(mod_to_run); // cargo run threaded 91 or 100
                } else {
                    println!("Error: Expected: 91/100");
                }
            }
            _ => {
                println!("Invalid number of arguments.");
                return;
            }
        }
    }
}

fn run_threaded(mod_to_run: &str) {
    match mod_to_run {
        "91" => threaded::mod_91::run(),
        "100" => threaded::mod_100::run(),
        _ => println!("Invalid mod number."),
    }
}
