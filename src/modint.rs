//mod modint {

use std;
use std::fmt;
use std::num::ParseIntError;
use std::ops;
use std::str::FromStr;

type ValueType = u64;
const MODNUM: ValueType = 1_000_000_007;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Modint {
    p: ValueType,
    v: ValueType,
}

impl Modint {
    #[allow(dead_code)]
    pub fn new(i: ValueType) -> Modint {
        Modint { p: MODNUM, v: i }
    }
}

impl FromStr for Modint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: ValueType;
        num = s.parse()?;
        Ok(Modint { p: MODNUM, v: num })
    }
}

//print用
impl fmt::Display for Modint {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}

//演算子関係
impl<'a> ops::Add for &'a Modint {
    type Output = Modint;

    #[allow(dead_code)]
    fn add(self, other: &'a Modint) -> Modint {
        if self.p != other.p {
            panic!("Mod Error");
        }
        Modint {
            p: self.p,
            v: (self.v + other.v) % self.p,
        }
    }
}

impl<'a> ops::Sub for &'a Modint {
    type Output = Modint;

    #[allow(dead_code)]
    fn sub(self, other: &'a Modint) -> Modint {
        if self.p != other.p {
            panic!("Mod Error");
        }
        Modint {
            p: self.p,
            v: (self.v + self.p - (other.v % self.p)) % self.p,
        }
    }
}

impl<'a> ops::Mul for &'a Modint {
    type Output = Modint;

    #[allow(dead_code)]
    fn mul(self, other: &'a Modint) -> Modint {
        if self.p != other.p {
            panic!("Mod Error");
        }
        Modint {
            p: self.p,
            v: (self.v * other.v) % self.p,
        }
    }
}

impl<'a> ops::Div for &'a Modint {
    type Output = Modint;

    fn div(self, other: &'a Modint) -> Modint {
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
    pub fn add_uint(&mut self, n: ValueType) -> &Self {
        self.v += n;
        self.v %= self.p;
        self
    }

    #[allow(dead_code)]
    pub fn sub_uint(&mut self, mut n: ValueType) -> &Self {
        n = n % self.p;
        self.v += self.p;
        self.v -= n;
        self.v %= self.p;
        self
    }

    #[allow(dead_code)]
    pub fn mul_uint(&mut self, n: ValueType) -> &Self {
        self.v *= n;
        self.v %= self.p;
        self
    }

    #[allow(dead_code)]
    pub fn div_uint(&mut self, n: ValueType) -> &Self {
        let mut obj = Modint::new(n);
        obj.inv();
        self.v *= obj.v;
        self.v %= self.p;
        self
    }

    #[allow(dead_code)]
    pub fn inv(&mut self) -> &Self {
        let p = self.p;
        self.pow(p as usize - 2);
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

    #[allow(dead_code)]
    pub fn get_value(&self) -> ValueType {
        self.v
    }
}

pub struct DPFactorial {
    normal: Vec<Modint>,
    inv: Vec<Modint>,
}
impl DPFactorial {
    #[allow(dead_code)]
    pub fn new() -> DPFactorial {
        let mut obj = DPFactorial {
            normal: Vec::new(),
            inv: Vec::new(),
        };
        obj.normal.push(Modint::new(1));
        obj.inv.push(Modint::new(1));
        obj
    }

    #[allow(dead_code)]
    pub fn get_factorial(&mut self, n: usize) -> Modint {
        if n < self.normal.len() {
            return self.normal[n].clone();
        }
        for z in self.normal.len()..n as usize + 1 {
            let buf = Modint::new(z as ValueType);
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
            let mut buf = Modint::new(z as ValueType);
            buf.inv();
            let buf = &buf * &self.inv[z - 1];
            self.inv.push(buf);
        }
        self.inv[n].clone()
    }

    #[allow(dead_code)]
    pub fn get_combination(&mut self, n: usize, r: usize) -> Modint {
        if n < r {
            return Modint::new(0);
        }
        &(&self.get_factorial(n) * &(self.get_factorial_inv(n - r))) * &(self.get_factorial_inv(r))
    }

    #[allow(dead_code)]
    pub fn get_permutation(&mut self, n: usize, r: usize) -> Modint {
        if n < r {
            return Modint::new(0);
        }
        &self.get_factorial(n) * &(self.get_factorial_inv(n - r))
    }
}
//}
