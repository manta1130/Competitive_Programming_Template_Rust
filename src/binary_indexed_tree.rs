//提出時は下と一番下のコメントアウトを外す。
//mod  binary_indexed_tree{

type VecType = u64;

pub struct BIT {
    v: Vec<VecType>,
}

impl BIT {
    pub fn new(n: usize) -> BIT {
        let v = vec![0; n + 1];
        BIT { v: v }
    }

    pub fn add(&mut self, index: usize, value: VecType) {
        let mut i = (index + 1) as isize;
        while i <= (self.v.len() - 1) as isize {
            self.v[i as usize] += value;
            i += i & -i;
        }
    }

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
