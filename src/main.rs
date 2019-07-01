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

fn main() {
      
}
