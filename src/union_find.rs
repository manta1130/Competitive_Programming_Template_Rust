//提出時は下と一番下のコメントアウトを外す。
//mod union_find {

///素集合データ構造
pub struct UnionFind {
    v: Vec<usize>,
    h: Vec<usize>,
}

impl UnionFind {
    ///要素数nの素集合データ構造を構築する。
    pub fn new(n: usize) -> UnionFind {
        let mut v = Vec::new();
        for i in 0..n {
            v.push(i);
        }
        let h = vec![1; n];
        UnionFind { v: v, h: h }
    }

    ///親を探索する。
    pub fn find(&mut self, a: usize) -> usize {
        if a == self.v[a] {
            return a;
        }

        let buf = self.v[a];
        let parent = self.find(buf);
        self.v[a] = parent;
        parent
    }

    ///親が同じか判定する。
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    ///結合する。
    pub fn union(&mut self, a: usize, b: usize) {
        let parent_a = self.find(a);
        let parent_b = self.find(b);
        if self.h[parent_a] < self.h[parent_b] {
            self.v[parent_a] = parent_b;
        } else {
            self.v[parent_b] = parent_a;

            if self.h[parent_a] == self.h[parent_b] {
                self.h[parent_a] += 1;
            }
        }
    }
}
//}
