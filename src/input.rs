//提出時は下二行と一番下のコメントアウトを外す。
//#[macro_use]
//mod input {

use std;
use std::io;

const SPLIT_DELIMITER: char = ' ';

/// 空白で区切られた複数の値の読み込む。
/// # Example
/// ```ignore
/// use cp_template::*;
///
/// let (a,b):(usize,usize);
/// input!(a,b);
/// ```
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

/// 文字列を一行読み込む
/// # Example
/// ```ignore
/// use cp_template::*;
///
/// let s = input_line_str();
/// ```
#[allow(dead_code)]
pub fn input_line_str() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

/// 一つの変数を出力する。
/// ```
/// use cp_template::*;
///
/// let a=100;
/// p(a);
/// //上と下は等価です。
/// println!("{}",a);
/// ```
#[allow(dead_code)]
pub fn p<T>(t: T)
where
    T: std::fmt::Display,
{
    println!("{}", t);
}

/// 指定した行数を読み込み、二次元配列に変換する。
/// # Examples
/// ```ignore
/// use cp_template::*;
///
/// //10行読み込む。
/// let v = input_vector2d::<usize>(10);
/// ```
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

/// 一行読み込み、配列(Vec)に変換する。
/// # Examples
/// ```ignore
/// use cp_template::*;
///
/// let v=input_vector::<usize>();
/// ```
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

///　指定された行数を読み込む
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

/// StringをVec<char>に変換するトレイト
pub trait ToCharVec {
    fn to_charvec(&self) -> Vec<char>;
}

impl ToCharVec for String {
    fn to_charvec(&self) -> Vec<char> {
        self.to_string().chars().collect::<Vec<_>>()
    }
}

//}
