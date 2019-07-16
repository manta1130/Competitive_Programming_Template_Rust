//mod modint {

use std;
use std::fmt;
use std::ops;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Modint {
    p: usize, //
    v: usize,
}

impl Modint {
    #[allow(dead_code)]
    pub fn new(p: usize) -> Modint {
        Modint { p, v: 0 }
    }
    #[allow(dead_code)]
    pub fn from(p: usize, v: usize) -> Modint {
        Modint { p, v: v % p }
    }
}

//print用
impl fmt::Display for Modint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}

//演算子関係
impl ops::Add for &Modint {
    type Output = Modint;

    #[allow(dead_code)]
    fn add(self, other: &Modint) -> Modint {
        if self.p != other.p {
            panic!("Mod Error");
        }
        Modint {
            p: self.p,
            v: (self.v + other.v) % self.p,
        }
    }
}

impl ops::Sub for &Modint {
    type Output = Modint;

    #[allow(dead_code)]
    fn sub(self, other: &Modint) -> Modint {
        if self.p != other.p {
            panic!("Mod Error");
        }
        Modint {
            p: self.p,
            v: (self.v + self.p - (other.v % self.p)) % self.p,
        }
    }
}

impl ops::Mul for &Modint {
    type Output = Modint;

    #[allow(dead_code)]
    fn mul(self, other: &Modint) -> Modint {
        if self.p != other.p {
            panic!("Mod Error");
        }
        Modint {
            p: self.p,
            v: (self.v * other.v) % self.p,
        }
    }
}

impl ops::Div for &Modint {
    type Output = Modint;

    fn div(self, other: &Modint) -> Modint {
        if self.p != other.p {
            panic!("Mod Error");
        }
        let mut obj = self.clone();
        obj.div_uint(other.v);
        obj
    }
}

//ToDo
//AddAssign,SubAssign,MulAssign,DivAssignを実装

impl Modint {
    #[allow(dead_code)]
    pub fn add_uint(&mut self, n: usize) -> &Self {
        self.v += n;
        self.v %= self.p;
        self
    }

    #[allow(dead_code)]
    pub fn sub_uint(&mut self, mut n: usize) -> &Self {
        n = n % self.p;
        self.v += self.p;
        self.v -= n;
        self.v %= self.p;
        self
    }

    #[allow(dead_code)]
    pub fn mul_uint(&mut self, n: usize) -> &Self {
        self.v *= n;
        self.v %= self.p;
        self
    }

    #[allow(dead_code)]
    pub fn div_uint(&mut self, n: usize) -> &Self {
        let mut obj = Modint::from(self.p, n);
        obj.inv();
        self.v *= obj.v;
        self.v %= self.p;
        self
    }

    #[allow(dead_code)]
    pub fn inv(&mut self) -> &Self {
        let p = self.p;
        self.pow(p - 2);
        self
    }

    #[allow(dead_code)]
    pub fn pow(&mut self, mut n: usize) -> &Self {
        let mut temp = self.v;
        self.v = 1;
        while n > 0 {
            if n & 1 != 0 {
                self.v *= temp;
                self.v %= self.p;
            }
            n >>= 1;
            temp *= temp;
            temp %= self.p;
        }
        self
    }
}

pub struct DPFactorial {
    p: usize,
    normal: Vec<Modint>,
    inv: Vec<Modint>,
}
impl DPFactorial {
    #[allow(dead_code)]
    pub fn new(p: usize) -> DPFactorial {
        let mut obj = DPFactorial {
            p,
            normal: Vec::new(),
            inv: Vec::new(),
        };
        obj.normal.push(Modint::from(p, 1));
        obj.inv.push(Modint::from(p, 1));
        obj
    }

    #[allow(dead_code)]
    pub fn get_factorial(&mut self, n: usize) -> Modint {
        if n < self.normal.len() {
            return self.normal[n].clone();
        }
        for z in self.normal.len()..n + 1 {
            let buf = Modint::from(self.p, z);
            let buf = &buf * &self.normal[z - 1];

            self.normal.push(buf);
        }
        return self.normal[n].clone();
    }

    #[allow(dead_code)]
    pub fn get_factorial_inv(&mut self, n: usize) -> Modint {
        if n < self.inv.len() {
            return self.inv[n].clone();
        }
        for z in self.inv.len()..n + 1 {
            let mut buf = Modint::from(self.p, z);
            buf.inv();
            let buf = &buf * &self.inv[z - 1];
            self.inv.push(buf);
        }
        self.inv[n].clone()
    }

    #[allow(dead_code)]
    pub fn get_combination(&mut self, n: usize, r: usize) -> Modint {
        if n < r {
            return Modint::from(self.p, 0);
        }
        &(&self.get_factorial(n) * &(self.get_factorial_inv(n - r))) * &(self.get_factorial_inv(r))
    }

    #[allow(dead_code)]
    pub fn get_permutation(&mut self, n: usize, r: usize) -> Modint {
        if n < r {
            return Modint::from(self.p, 0);
        }
        &self.get_factorial(n) * &(self.get_factorial_inv(n - r))
    }
}
//}
