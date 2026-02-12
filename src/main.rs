mod evaluator;
mod parser;
mod printer;
mod scanner;
mod types;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn repl() {
    println!("Welcome to Linked Lisp! Have fun lisping!");

    loop {
        let mut input = String::new();

        print!("linked> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Linked: Failed to read input!\n");
        printer::print(evaluator::evaluate(parser::parse(scanner::scan(&input))));
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: linked [path]");
        process::exit(1);
    } else if args.len() == 2 {
        let contents =
            fs::read_to_string(args[1].clone()).expect("Linked: Error reading the file!\n");

        println!("{:?}", scanner::scan(&contents));
    // RUN PARSE HERE!
    } else {
        repl();
    }
}
