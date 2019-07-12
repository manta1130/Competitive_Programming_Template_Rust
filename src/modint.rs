use std::fmt;
use std::ops;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Modint {
    p: usize, //modint
    v: usize,
}
impl Modint {
    fn new(p: usize, v: usize) -> Modint {
        Modint { p, v }
    }
}

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

impl Modint {
    #[allow(dead_code)]
    fn add_uint(&mut self, n: usize) -> &Self {
        self.v += n;
        self.v %= self.p;
        self
    }
    #[allow(dead_code)]
    fn sub_uint(&mut self, mut n: usize) -> &Self {
        n = n % self.p;
        self.v += self.p;
        self.v -= n;
        self.v %= self.p;
        self
    }
    #[allow(dead_code)]
    fn mul_uint(&mut self, n: usize) -> &Self {
        self.v *= n;
        self.v %= self.p;
        self
    }
    #[allow(dead_code)]
    fn div_uint(&mut self, n: usize) -> &Self {
        let mut obj = Modint::new(self.p, n);
        obj.inv();
        self.v *= obj.v;
        self.v %= self.p;
        self
    }
    #[allow(dead_code)]
    fn inv(&mut self) -> &Self {
        let p = self.p;
        self.pow(p - 2);
        self
    }
    #[allow(dead_code)]
    fn pow(&mut self, mut n: usize) -> &Self {
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
