use std::io::{ Write };
use std::env;

extern crate console;

mod scanner;
mod parser;
use parser::Parser;


use std::collections::HashMap;

mod terminal_history;
mod terminal;

use terminal::Terminal;

fn compute(exp_str: &str) -> Result<f64, String> {
    let mut r: Result<f64, String> = Ok(0.);
    for res in Parser::new(exp_str) {
        match res {
            Result::Err(s) => r = Err(s),
            Result::Ok(v) => r = Ok(v),
        }
    }
    r
}

fn main() {
    let mut terminal = Terminal::new(String::from(">>> "), 16);
    terminal.clear();

    loop {
        let exp = terminal.read_line();
        let exp_str = exp.as_str();
        println!("exp_str = {}", exp_str);

        match exp_str {
            "exit" => break,
            "help" => print_help(),
            "clear" => terminal.clear(),

            _ => {
                match compute(exp_str) {
                    Result::Ok(v) => println!("Result = {}", v),
                    Result::Err(err) => println!("Error = {}", err),
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::compute;

    #[test]
    fn calculator_tests() {
        let mut map = HashMap::new();
        map.insert("min(1, 2, 3, sum(1, 5))", 1.);
        map.insert("min(0) + min(1)", 1.);
        map.insert("min(min(1, 2), 3)", 1.);
        map.insert("min(max(-4, -2), 3)", -2.);
        map.insert("1 + 2", 3.);

        for exp in map {
            let result = compute(exp.0).unwrap();
            println!("Expression: {}\nExpected: {}\nResult: {}\n\n", exp.0, exp.1, result);
            assert_eq!(exp.1, result);
        }
    }
}