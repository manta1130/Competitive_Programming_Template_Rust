//提出時は下と一番下のコメントアウトを外す。
//mod math {

use std;
use std::cmp;
use std::ops;

///最大公約数を求める。
pub fn gcd<T>(a: T, b: T) -> T
where
    T: ops::Add<Output = T> + ops::Rem<Output = T> + ops::Div<Output = T> + cmp::PartialEq + Copy,
{
    if b + b == b {
        return a;
    }
    gcd(b, a % b)
}

///最小公倍数を求める。
pub fn lcm<T>(a: T, b: T) -> T
where
    T: ops::Add<Output = T>
        + ops::Rem<Output = T>
        + ops::Div<Output = T>
        + ops::Mul<Output = T>
        + cmp::PartialEq
        + Copy,
{
    (a / gcd(a, b)) * b
}

//}
