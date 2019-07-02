use std::io;

fn main() {

}

#[allow(unused_macros)]
macro_rules! input {
    ( $($x:expr ),*) => {
        {
            let temp_str = input_line_str();
            let mut split_result_iter = temp_str.split_whitespace();
                $(
                let buf_split_result = split_result_iter.next();
                let buf_split_result = buf_split_result.unwrap();
                    ($x) = buf_split_result.parse().unwrap();
                )*
        }
    };
}

#[allow(dead_code)]
fn input_line_str() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

#[allow(dead_code)]
fn p<T>(t: T)
where
    T: std::fmt::Display,
{
    println!("{}", t);
}

const SPLIT_DELIMITER: char = ' ';
#[allow(dead_code)]
fn input_vector2d<T>(line: usize) -> Vec<Vec<T>>
where
    T: std::str::FromStr,
{
    let mut v: Vec<Vec<T>> = Vec::new();

    for _ in 0..line {
        let vec_line = input_vector();
        v.push(vec_line);
    }
    v
}

#[allow(dead_code)]
fn input_vector<T>() -> Vec<T>
where
    T: std::str::FromStr,
{
    let mut v: Vec<T> = Vec::new();

    let s = input_line_str();
    let split_result = s.split(SPLIT_DELIMITER);
    for z in split_result {
        let buf = match z.parse() {
            Ok(r) => r,
            Err(_) => panic!("Parse Error"),
        };
        v.push(buf);
    }
    v
}

#[allow(dead_code)]
fn str2vec(s: &str) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    for c in s.chars() {
        v.push(c);
    }
    v
}

fn input_vector<T>(line: usize) -> Vec<Vec<T>>
where
    T: std::str::FromStr,
{
    let mut v: Vec<Vec<T>> = Vec::new();

    for _ in 0..line {
        let s = input_line_str();
        let split_result = s.split_whitespace();
        let mut vec_line: Vec<T> = Vec::new();
        for z in split_result {
            let buf = match z.parse() {
                Ok(r) => r,
                Err(_) => panic!("Parse Error"),
            };
            vec_line.push(buf);
        }
        v.push(vec_line);
    }
    v
}
