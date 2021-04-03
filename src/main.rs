extern crate regex;

mod lexer;
use std::io::{self, Write};

fn main() {
    println!("Welcome to rust calculator");

    loop {
        print!(">");
        io::stdout().flush().unwrap();

        let mut statement = String::new();
        let op_stdin = io::stdin();
        op_stdin.read_line(&mut statement).unwrap();

        if statement.to_string().trim() == "exit".to_string() {
            println!("Bye Bye");
            break;
        }

        let result: String = lexer::start(&mut statement);
        println!("{}", result);
    }

    return;
}
