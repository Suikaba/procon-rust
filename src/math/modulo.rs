use std::fmt;
use std::ops::{Add, Mul, Sub, Div};

#[allow(dead_code)]
const MODLL: i64 = 1000_000_007;

#[derive(Copy, Clone)]
struct Modulo {
    n: i64
}

// precondition: p is prime and mod
fn mod_inverse(a: i64, p: i64) -> i64 {
    if a == 1 {
        1
    } else {
        (1 - p * mod_inverse(p % a, a)) / a + p
    }
}

#[allow(dead_code)]
impl Modulo {
    fn new(n: i64) -> Modulo {
        Modulo { n: (n % MODLL + MODLL) % MODLL }
    }

    fn inv(&self) -> Modulo {
        Modulo { n: mod_inverse(self.n, MODLL) }
    }
}

impl fmt::Debug for Modulo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.n)
    }
}

impl Add for Modulo {
    type Output = Modulo;
    fn add(self, other: Modulo) -> Modulo {
        Modulo { n: (self.n + other.n) % MODLL }
    }
}
impl Add<i32> for Modulo {
    type Output = Modulo;
    fn add(self, other: i32) -> Modulo {
        Modulo { n: (self.n + other as i64) % MODLL }
    }
}
impl Add<i64> for Modulo {
    type Output = Modulo;
    fn add(self, other: i64) -> Modulo {
        Modulo { n: (self.n + other) % MODLL }
    }
}
impl Sub for Modulo {
    type Output = Modulo;
    fn sub(self, other: Modulo) -> Modulo {
        Modulo { n: (self.n - other.n + MODLL) % MODLL }
    }
}
impl Sub<i32> for Modulo {
    type Output = Modulo;
    fn sub(self, other: i32) -> Modulo {
        self - Modulo::new(other as i64)
    }
}
impl Sub<i64> for Modulo {
    type Output = Modulo;
    fn sub(self, other: i64) -> Modulo {
        self - Modulo::new(other)
    }
}
impl Mul for Modulo {
    type Output = Modulo;
    fn mul(self, other: Modulo) -> Modulo {
        Modulo { n: (self.n * other.n) % MODLL }
    }
}
impl Mul<i32> for Modulo {
    type Output = Modulo;
    fn mul(self, other: i32) -> Modulo {
        self * Modulo::new(other as i64)
    }
}
impl Mul<i64> for Modulo {
    type Output = Modulo;
    fn mul(self, other: i64) -> Modulo {
        self * Modulo::new(other)
    }
}
impl Div for Modulo {
    type Output = Modulo;
    fn div(self, other: Modulo) -> Modulo {
        self * other.inv()
    }
}
impl Div<i32> for Modulo {
    type Output = Modulo;
    fn div(self, other: i32) -> Modulo {
        self * mod_inverse(other as i64, MODLL)
    }
}
impl Div<i64> for Modulo {
    type Output = Modulo;
    fn div(self, other: i64) -> Modulo {
        self * mod_inverse(other, MODLL)
    }
}

struct Combination {
    fact: Vec<Modulo>,
    ifact: Vec<Modulo>
}

#[allow(dead_code)]
impl Combination {
    fn new(n: usize) -> Combination {
        let mut fact = vec![Modulo::new(1); n + 1];
        let mut ifact = vec![Modulo::new(1); n + 1];
        for i in 1..n+1 {
            fact[i] = fact[i - 1] * i as i64;
            ifact[i] = ifact[i - 1] / i as i64;
        }
        Combination {
            fact: fact,
            ifact: ifact
        }
    }

    /// n!
    fn fact(&self, n: usize) -> Modulo {
        self.fact[n]
    }
    /// 1/n!
    fn inv_fact(&self, n: usize) -> Modulo {
        self.ifact[n]
    }
    /// nCr
    fn comb(&self, n: usize, r: usize) -> Modulo {
        if r > n {
            Modulo::new(1)
        } else {
            self.fact(n) * self.inv_fact(n - r) * self.inv_fact(r)
        }
    }
}