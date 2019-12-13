//提出時は下と一番下のコメントアウトを外す。
//mod  rolling_hash{

use std::num::Wrapping;

type ValueType = u64;

pub struct RollingHash {
    //base: ValueType,
    v: Vec<ValueType>,
    pow: Vec<ValueType>,
}

///ローリングハッシュ
impl RollingHash {
    ///ローリングハッシュを計算する。
    ///
    /// base:基数
    pub fn calc(input_str: &str, base: ValueType) -> RollingHash {
        let mut v = vec![0];
        let mut pow = vec![1];
        let mut pow_buf = base;
        let mut hash: ValueType = 0;

        for _ in 0..input_str.len() {
            pow.push(pow_buf);
            pow_buf = (Wrapping(pow_buf) * Wrapping(base)).0;
        }

        for c in input_str.as_bytes() {
            hash = (Wrapping(base) * Wrapping(hash)).0;
            hash = (Wrapping(hash) + Wrapping(*c as ValueType)).0;
            v.push(hash);
        }
        RollingHash {
            //base: base,
            v: v,
            pow: pow,
        }
    }

    ///[from:to)のローリングハッシュを求める。
    pub fn get(&self, from: usize, to: usize) -> ValueType {
        let buf = (Wrapping(self.v[from]) * Wrapping(self.pow[to - from])).0;
        (Wrapping(self.v[to]) - Wrapping(buf)).0
    }
}
//}
