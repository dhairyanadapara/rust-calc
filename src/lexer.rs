use regex::Regex;

mod lexer {
    pub fn start() {}
}

pub enum Token {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    NUMBER,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn S3(statement: &mut str) {
    let re = Regex::new(r"(\d+\-)+\d").unwrap();

    let mut evolved_statement = statement.to_string();
    loop {
        let t1: &str = match re.captures(&evolved_statement) {
            Some(cap) => cap.get(0).map_or("", |m| m.as_str()),
            None => "",
        };

        if t1 == "" {
            break;
        }
        let v: Vec<i32> = t1
            .split("-")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        let rem: i32 = v[1..v.len()].iter().sum();
        let v1: i32 = v[0] - rem;

        evolved_statement = evolved_statement.replace(t1, &v1.to_string());
    }
    println!("sub {}: {}", statement, evolved_statement);
}

fn S2(statement: &mut str) {
    let re = Regex::new(r"(\d+\+)+\d").unwrap();
    let mut evolved_statement = statement.to_string();
    loop {
        let t1: &str = match re.captures(&evolved_statement) {
            Some(cap) => cap.get(0).map_or("", |m| m.as_str()),
            None => "",
        };

        if t1 == "" {
            break;
        }
        let v: i32 = t1
            .split("+")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .sum();
        evolved_statement = evolved_statement.replace(t1, &v.to_string());
    }
    println!("add {}: {}", statement, evolved_statement);
}

fn S1(statement: &mut str) {
    let re = Regex::new(r"(\d+\*)+\d").unwrap();

    let mut evolved_statement = statement.to_string();
    loop {
        let t1: &str = match re.captures(&evolved_statement) {
            Some(cap) => cap.get(0).map_or("", |m| m.as_str()),
            None => "",
        };

        if t1 == "" {
            break;
        }
        let v: i32 = t1
            .split("*")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .product();
        evolved_statement = evolved_statement.replace(t1, &v.to_string());
    }
    println!("mul {}: {}", statement, evolved_statement);
}

fn S0(statement: &mut str) {
    let re = Regex::new(r"(\d+/)+\d").unwrap();

    let mut evolved_statement = statement.to_string();
    loop {
        let t1: &str = match re.captures(&evolved_statement) {
            Some(cap) => cap.get(0).map_or("", |m| m.as_str()),
            None => "",
        };

        if t1 == "" {
            break;
        }
        let v: Vec<i32> = t1
            .split("/")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();

        let rem: i32 = v[1..v.len()].iter().product();
        let v1: i32 = v[0] / rem;

        evolved_statement = evolved_statement.replace(t1, &v1.to_string());
    }
    println!("divide {}: {}", statement, evolved_statement);
    statement = evolved_statement;
}

pub fn start(statement: &mut str) {
    println!("{:?}", statement);
    S0(statement);
    S1(statement);
    S2(statement);
    S3(statement);
}
