//  ____            _                  _                       _       _
// |  _ \ _   _ ___| |_    ___ _ __   | |_ ___ _ __ ___  _ __ | | __ _| |_ ___
// | |_) | | | / __| __|  / __| '_ \  | __/ _ \ '_ ` _ \| '_ \| |/ _` | __/ _ \
// |  _ <| |_| \__ \ |_  | (__| |_) | | ||  __/ | | | | | |_) | | (_| | ||  __/
// |_| \_\\__,_|___/\__|  \___| .__/___\__\___|_| |_| |_| .__/|_|\__,_|\__\___|
//                            |_| |_____|               |_|

//                                                     _        _ _ _____  ___
//                               _ __ ___   __ _ _ __ | |_ __ _/ / |___ / / _ \
//                              | '_ ` _ \ / _` | '_ \| __/ _` | | | |_ \| | | |
//                              | | | | | | (_| | | | | || (_| | | |___) | |_| |
//                              |_| |_| |_|\__,_|_| |_|\__\__,_|_|_|____/ \___/

//https://github.com/manta1130/Competitive_Programming_Template_Rust

#[macro_use]
mod input {

    use std;
    use std::io;

    const SPLIT_DELIMITER: char = ' ';

    #[macro_export]
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
    pub fn input_line_str() -> String {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.trim().to_string()
    }

    #[allow(dead_code)]
    pub fn p<T>(t: T)
    where
        T: std::fmt::Display,
    {
        println!("{}", t);
    }

    #[allow(dead_code)]
    pub fn input_vector2d<T>(line: usize) -> Vec<Vec<T>>
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
    pub fn input_vector<T>() -> Vec<T>
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
    pub fn input_vector_row<T>(n: usize) -> Vec<T>
    where
        T: std::str::FromStr,
    {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            let buf = match input_line_str().parse() {
                Ok(r) => r,
                Err(_) => panic!("Parse Error"),
            };
            v.push(buf);
        }
        v
    }

    pub trait ToCharVec {
        fn to_charvec(&self) -> Vec<char>;
    }

    impl ToCharVec for String {
        fn to_charvec(&self) -> Vec<char> {
            self.to_string().chars().collect::<Vec<_>>()
        }
    }
}
