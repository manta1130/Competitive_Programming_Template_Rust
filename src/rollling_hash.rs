//提出時は下と一番下のコメントアウトを外す。
//mod  rolling_hash{

type ValueType = u64;

pub struct RollingHash {
    base: ValueType,
    v: Vec<ValueType>,
    pow: Vec<ValueType>,
}

impl RollingHash {
    pub fn new(input_str: &String, base: ValueType) -> RollingHash {
        let mut v = vec![];
        let mut pow = vec![0];
        let mut pow_buf = base;
        let mut hash: ValueType = 0;

        for _ in 0..input_str.len() {
            pow.push(pow_buf);
            pow_buf *= pow_buf;
        }

        for c in input_str.as_bytes() {
            hash *= base;
            hash += *c as ValueType;
            v.push(hash);
        }
        RollingHash {
            base: base,
            v: v,
            pow: pow,
        }
    }
}
//}
