trait Group {
    /// Data type
    type T: Clone;

    /// Identity function
    fn id() -> Self::T;

    /// Operator: op(a, b) <-> ab in this order
    fn op(a: &Self::T, b: &Self::T) -> Self::T;

    /// Inverse
    fn inverse(a: &Self::T) -> Self::T;
}

struct FenwickTree<G: Group> {
    data: std::vec::Vec<G::T>,
    size: usize
}

#[allow(dead_code)]
impl<G: Group> FenwickTree<G> {
    fn new(n: usize) -> FenwickTree<G> {
        FenwickTree::<G> {
            data: vec![G::id(); n],
            size: n,
        }
    }

    fn operate(&mut self, mut i: usize, val: G::T) {
        while i < self.size {
            self.data[i] = G::op(&self.data[i], &val);
            i |= i + 1;
        }
    }

    /// [0..r]
    fn query1(&self, mut r: i32) -> G::T {
        let mut res = G::id();
        while r >= 0 {
            res = G::op(&res, &self.data[r as usize]);
            r = (r & (r + 1)) - 1;
        }
        res
    }
    /// [l..r)
    fn query(&self, l: usize, r: usize) -> G::T {
        let (l, r) = (l as i32, r as i32);
        G::op(&self.query1(r - 1), &G::inverse(&self.query1(l - 1)))
    }
}

/// Example: Range Sum
struct RangeSum;
#[allow(dead_code)]
impl Group for RangeSum {
    type T = i32;
    fn id() -> i32 {
        0
    }
    fn op(a: &i32, b: &i32) -> i32 {
        a + b
    }
    fn inverse(a: &i32) -> i32 {
        -a
    }
}