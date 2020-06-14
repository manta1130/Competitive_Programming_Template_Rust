//提出時は下と一番下のコメントアウトを外す。
//mod math {

use std;
use std::cmp;
use std::default;
use std::ops;

///最大公約数を求める。
pub fn getgcd<T>(a: T, b: T) -> T
where
    T: ops::Add<Output = T> + ops::Rem<Output = T> + ops::Div<Output = T> + cmp::PartialEq + Copy,
{
    if b + b == b {
        return a;
    }
    getgcd(b, a % b)
}

///最小公倍数を求める。
pub fn getlcm<T>(a: T, b: T) -> T
where
    T: ops::Add<Output = T>
        + ops::Rem<Output = T>
        + ops::Div<Output = T>
        + ops::Mul<Output = T>
        + cmp::PartialEq
        + Copy,
{
    (a / getgcd(a, b)) * b
}

///拡張ユーグリッドの互除法
pub fn extgcd<T>(a: T, b: T, x: &mut T, y: &mut T) -> T
where
    T: ops::Add<Output = T>
        + ops::SubAssign
        + ops::Rem<Output = T>
        + ops::Div<Output = T>
        + ops::Mul<Output = T>
        + cmp::PartialEq
        + default::Default
        + From<bool>
        + Copy,
{
    if b == T::default() {
        *y = T::default();
        *x = T::from(true);
        return a;
    }
    let d = extgcd(b, a % b, y, x);
    *y -= a / b * *x;
    return d;
}

//}
