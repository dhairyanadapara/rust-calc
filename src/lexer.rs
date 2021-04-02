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

    let y = re.captures(statement).ok_or("");
    println!("{:?}", y);
}

fn S2(statement: &mut str) {
    
    let re = Regex::new(r"(\d+\+)+\d").unwrap();
    let mut evolved_statement = statement.to_string();
    loop {
        let t1: &str = match re.captures(&evolved_statement) {
            Some(cap) => cap.get(0).map_or("", |m| m.as_str()),
            None => ""
        };

        if(t1 == "") {
            break;
        }
        let v: i32 = t1.split("+").map(|x| x.trim().parse::<i32>().unwrap()).sum();
        evolved_statement = evolved_statement.replace(t1, &v.to_string());
    }

}

fn S1(statement: &mut str) {
    let re = Regex::new(r"(\d+\*)+\d").unwrap();

    let y = re.captures(statement).ok_or("");
    println!("{:?}", y);
}

fn S0(statement: &mut str) {
    let re = Regex::new(r"(\d+\\)+\d").unwrap();

    let y = re.captures(statement).ok_or("");
    println!("{:?}", y);
}

pub fn start(statement: &mut str) {
    S0(statement);
    S1(statement);
    S2(statement);
    S3(statement);
}
