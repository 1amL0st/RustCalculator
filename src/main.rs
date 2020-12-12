use std::io::{ Read, Write};
use std::env;

mod scanner;
mod parser;
use parser::Parser;

fn main() {
    loop {
        print!(">>> ");
        std::io::stdout().flush().expect("Unknow error while flushing stdin!");

        let mut exp = String::new();
        std::io::stdin().read_line(&mut exp).expect("Can't read line!");
        exp = exp.trim().to_string();

        let exp_str = exp.as_str();

        match exp_str {
            "exit" => break,
            "help" => print_help(),
            _ => {
                println!("Expression = {}", exp_str);

                for res in Parser::new(exp_str) {
                    match res {
                        Result::Err(s) => {
                            println!("{}", s);
                            break;
                        }
                        Result::Ok(v) => {
                            println!("{}", v);
                        }
                    }
                }
            }
        }
    }
}

fn print_help() {
    let exe = env::args().next().unwrap();
    println!("Commandline calculator written in Rust.");
    println!("You have power operator 2 ** 2 is equal to 2 ^ 2");
    println!("");
    println!("Try running it with something to calculate!");
    println!("Example:    {} 2 + 2 - cos pi", exe);
    println!("");
    println!("For more information see: https://github.com/Aggrathon/RustCalculator");
}
