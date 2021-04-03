use regex::Regex;

fn subtraction(statement: String) -> String {
    let re = Regex::new(r"([0-9]*[.]?[0-9]+\-)+[0-9]*[.]?[0-9]+").unwrap();

    let mut evolved_statement = statement.to_string();
    loop {
        let t1: &str = match re.captures(&evolved_statement) {
            Some(cap) => cap.get(0).map_or("", |m| m.as_str()),
            None => "",
        };

        if t1 == "" {
            break;
        }
        let v: Vec<f32> = t1
            .split("-")
            .map(|x| x.trim().parse::<f32>().unwrap())
            .collect();
        let rem: f32 = v[1..v.len()].iter().sum();
        let v1: f32 = v[0] - rem;

        evolved_statement = evolved_statement.replace(t1, &v1.to_string());
    }
    evolved_statement
}

fn addition(statement: String) -> String {
    let re = Regex::new(r"([0-9]*[.]?[0-9]+\+)+[0-9]*[.]?[0-9]+").unwrap();
    let mut evolved_statement = statement.to_string();
    loop {
        let t1: &str = match re.captures(&evolved_statement) {
            Some(cap) => cap.get(0).map_or("", |m| m.as_str()),
            None => "",
        };

        if t1 == "" {
            break;
        }
        let v: f32 = t1
            .split("+")
            .map(|x| x.trim().parse::<f32>().unwrap())
            .sum();
        evolved_statement = evolved_statement.replace(t1, &v.to_string());
    }
    evolved_statement
}

fn multiplication(statement: String) -> String {
    let re = Regex::new(r"([0-9]*[.]?[0-9]+\*)+[0-9]*[.]?[0-9]+").unwrap();

    let mut evolved_statement = statement.to_string();
    loop {
        let t1: &str = match re.captures(&evolved_statement) {
            Some(cap) => cap.get(0).map_or("", |m| m.as_str()),
            None => "",
        };

        if t1 == "" {
            break;
        }
        let v: f32 = t1
            .split("*")
            .map(|x| x.trim().parse::<f32>().unwrap())
            .product();
        evolved_statement = evolved_statement.replace(t1, &v.to_string());
    }
    evolved_statement
}

fn division(statement: String) -> String {
    let re = Regex::new(r"([0-9]*[.]?[0-9]+/)+[0-9]*[.]?[0-9]+").unwrap();

    let mut evolved_statement = statement.to_string();
    loop {
        let t1: &str = match re.captures(&evolved_statement) {
            Some(cap) => cap.get(0).map_or("", |m| m.as_str()),
            None => "",
        };

        if t1 == "" {
            break;
        }
        let v: Vec<f32> = t1
            .split("/")
            .map(|x| x.trim().parse::<f32>().unwrap())
            .collect();

        let rem: f32 = v[1..v.len()].iter().product();
        let v1: f32 = v[0] / rem;

        evolved_statement = evolved_statement.replace(t1, &v1.to_string());
    }
    evolved_statement
}

pub fn start(statement: &mut str) -> String {
    let mut val = statement.to_string();
    val = division(val);
    val = multiplication(val);
    val = addition(val);
    val = subtraction(val);
    val
}
