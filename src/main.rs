use std::io;

fn input_line_str() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn input_line_number<T>() -> T
where
    T: std::str::FromStr,
{
    let s = input_line_str();
    match s.parse() {
        Ok(r) => r,
        Err(_) => panic!("Parse Error"),
    }
}

fn main() {
    
}
