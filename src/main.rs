extern crate regex;

mod lexer;
use std::io::{self, Write};

fn addition(num1: i32, num2: i32) -> f64 {
    return f64::from(num1 + num2);
}

fn subtraction(num1: i32, num2: i32) -> f64 {
    return f64::from(num1 - num2);
}

fn multiplication(num1: i32, num2: i32) -> f64 {
    return f64::from(num1 * num2);
}

fn division(num1: i32, num2: i32) -> f64 {
    if num2 == 0 {
        println!("Invalid num2");
        return 0.0;
    }
    return f64::from(num1 / num2);
}

fn get_result<'a>(statement: &'a str, operator: &'a str) -> Vec<&'a str> {
    statement.trim().split(operator).map(|x| x.trim()).collect()
}

fn main() {
    println!("Hello, world!");

    print!(">");
    io::stdout().flush().unwrap();

    let mut statement = String::new();
    let op_stdin = io::stdin();
    // let mut final_list = Vec::new();
    op_stdin.read_line(&mut statement).unwrap();

    // let mut sub: Vec<&str> = statement.trim().split("-").map(|x| x.trim()).collect();
    // let add: Vec<&str> = statement.trim().split("+").map(|x| x.trim()).collect();
    // let multi: Vec<&str> = statement.trim().split("*").map(|x| x.trim()).collect();
    // let div: Vec<&str> = statement.trim().split("/").map(|x| x.trim()).collect();
    // let pow: Vec<&str> = statement.trim().split("**").map(|x| x.trim()).collect();

    lexer::start(&mut statement);
    // let x: Vec<Vec<&str>> = sub
    //     .iter()
    //     .map(|x| x.trim().split("+").map(|x| x.trim()))
    //     .collect();

    // sub.push("-");
    // final_list.push(sub);

    // println!("{:?}", final_list);

    // println!("{:?}", x);

    // println!("{:?}", sub);
    // println!("{:?}", add);
    // println!("{:?}", multi);
    // println!("{:?}", div);
    // println!("{:?}", pow);
    // println!("{:?}", add);

    return;
    // let op = match operation.trim().parse::<i32>() {
    //     Err(e) => {
    //         println!("Error {}", e);
    //         0
    //     }
    //     Ok(i) => {
    //         println!("Integer {}", i);
    //         i
    //     }
    // };

    // print!("Please enter 1st number: ");
    // io::stdout().flush().unwrap();
    // let mut num1 = String::new();
    // let input1 = io::stdin();
    // input1.read_line(&mut num1).unwrap();
    // let n1 = num1.trim().parse::<i32>().unwrap();

    // print!("Please enter 2nd number: ");
    // io::stdout().flush().unwrap();
    // let mut num2 = String::new();
    // let input2 = io::stdin();
    // input2.read_line(&mut num2).unwrap();
    // let n2 = num2.trim().parse::<i32>().unwrap();

    // let result = match op {
    //     1 => addition(n1, n2),
    //     2 => subtraction(n1, n2),
    //     3 => multiplication(n1, n2),
    //     4 => division(n1, n2),
    //     _ => 0.0,
    // };

    // println!("Result: {}", result);
}
