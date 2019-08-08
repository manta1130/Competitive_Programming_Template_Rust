//提出時は下二行と一番下のコメントアウトを外す。
//#[macro_use]
//mod prime_number {

use std::iter::Iterator;

pub struct PrimeFactorization {
    n: usize,
    cur: usize,
    limit: usize,
}

///素因数分解
impl PrimeFactorization {
    ///素因数を計算するイテレータを返す。
    pub fn calc(n: usize) -> PrimeFactorization {
        PrimeFactorization {
            n: n,
            cur: 1,
            limit: (n as f64).sqrt() as usize + 1,
        }
    }
}

impl Iterator for PrimeFactorization {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.cur == 0 || self.cur > self.n {
                return None;
            }

            self.cur += 1;

            if self.cur > self.limit {
                if self.n != 1 {
                    self.cur = 0;
                    return Some(self.n);
                }
                return None;
            }
            if self.n % self.cur == 0 {
                self.n /= self.cur;
                self.cur -= 1;
                return Some(self.cur + 1);
            }
        }
    }
}

//}
