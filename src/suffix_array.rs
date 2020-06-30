//提出時は下と一番下のコメントアウトを外す。
//mod suffix_array{
use std::cmp::Ordering;

///SuffixArrayの計算を行う(Manber&Myersのアルゴリズム)
pub struct SuffixArray {
    n: usize,
    k: usize,
    rank: Vec<isize>,
    s: Vec<isize>,
}

impl SuffixArray {
    pub fn calc(s: &str) -> Vec<usize> {
        let mut obj = SuffixArray {
            n: s.len(),
            k: 1,
            rank: vec![0; s.len() + 1],
            s: s.as_bytes()
                .iter()
                .map(|&x| x as isize)
                .collect::<Vec<isize>>(),
        };
        let mut sa = vec![0; s.len() + 1];
        let mut tmp = vec![0; s.len() + 1];

        for i in 0..=obj.n {
            sa[i] = i;
            obj.rank[i] = if i < obj.n { obj.s[i] } else { -1 };
        }

        while obj.k <= obj.n {
            sa.sort_by(|a, b| obj.comp(*a, *b));

            tmp[sa[0]] = 0;
            for i in 1..=obj.n {
                tmp[sa[i]] = tmp[sa[i - 1]]
                    + if let Ordering::Less = obj.comp(sa[i - 1], sa[i]) {
                        1
                    } else {
                        0
                    };
            }
            for i in 0..=obj.n {
                obj.rank[i] = tmp[i]
            }

            obj.k *= 2;
        }

        sa
    }
    fn comp(&self, i: usize, j: usize) -> Ordering {
        if self.rank[i] != self.rank[j] {
            self.rank[i].cmp(&self.rank[j])
        } else {
            let ri = if i + self.k <= self.n {
                self.rank[i + self.k]
            } else {
                -1
            };
            let rj = if j + self.k <= self.n {
                self.rank[j + self.k]
            } else {
                -1
            };
            ri.cmp(&rj)
        }
    }
}
//}
