//提出時は下と一番下のコメントアウトを外す。
//mod  binary_indexed_tree{

type VecType = u64;

///Binary Indexed Tree
pub struct BIT {
    v: Vec<VecType>,
}

impl BIT {
    ///BIT木を構築する。
    ///
    ///n:要素数
    pub fn new(n: usize) -> BIT {
        let v = vec![0; n + 1];
        BIT { v: v }
    }

    ///任意の要素に値を加算する。
    ///
    /// index:加算する場所(0-indexed)
    /// value:加算する値
    pub fn add(&mut self, index: usize, value: VecType) {
        let mut i = (index + 1) as isize;
        while i <= (self.v.len() - 1) as isize {
            self.v[i as usize] += value;
            i += i & -i;
        }
    }

    ///[0,index]の区間和を求める。
    pub fn get(&mut self, index: usize) -> VecType {
        let mut i = (index + 1) as isize;
        let mut res = 0 as VecType;
        while i > 0 {
            res += self.v[i as usize];
            i -= i & -i;
        }
        res
    }
}

//}
