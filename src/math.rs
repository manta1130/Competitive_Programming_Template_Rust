//提出時は下と一番下のコメントアウトを外す。
//mod math {

use std;
use std::cmp;
use std::default;
use std::ops;

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
