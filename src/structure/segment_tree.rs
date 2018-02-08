#[allow(dead_code)]
trait Monoid {
    /// Data type
    type T: Clone;

    /// Identity function
    fn id() -> Self::T;

    /// Operator: op(a, b) <-> ab in this order
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}

/// helper
fn expand(n: usize) -> usize {
    assert!(n >= 1);
    match n {
        1 => 1,
        n => expand((n + 1) / 2) * 2,
    }
}

struct SegmentTree<M: Monoid> {
    data: std::vec::Vec<M::T>,
    size: usize,
    n: usize,
}

#[allow(dead_code)]
impl<M: Monoid> SegmentTree<M> {
    fn new(n: usize) -> SegmentTree<M> {
        let sz = n;
        let n = expand(sz);
        SegmentTree::<M> {
            data: vec![M::id(); n * 2],
            size: sz,
            n: n,
        }
    }
    fn new_from_vec(data: &std::vec::Vec<M::T>) -> SegmentTree<M> {
        let sz = data.len();
        let n = expand(sz);
        let mut dat: std::vec::Vec<M::T> = vec![M::id(); n * 2];
        for i in 0..sz {
            dat[n + i] = data[i].clone();
        }
        for i in (1..n).rev() {
            dat[i] = M::op(&dat[i * 2], &dat[i * 2 + 1]);
        }

        SegmentTree::<M> {
            data: dat,
            size: sz,
            n: n,
        }
    }

    fn update(&mut self, mut p: usize, val: M::T) {
        assert!(p < self.size);
        p += self.n;
        self.data[p] = val;
        p /= 2;
        while p > 0 {
            self.data[p] = M::op(&self.data[p * 2], &self.data[p * 2 + 1]);
            p /= 2;
        }
    }

    fn query(&self, mut l: usize, mut r: usize) -> M::T {
        assert!(l < r && r <= self.size);
        let n = self.n;
        l += n; r += n;
        let mut res1 = M::id();
        let mut res2 = M::id();
        while l != r {
            if l & 1 > 0 {
                res1 = M::op(&res1, &self.data[l]);
                l += 1;
            }
            if r & 1 > 0 {
                r -= 1;
                res2  = M::op(&self.data[r], &res2);
            }
            l /= 2; r /= 2;
        }
        M::op(&res1, &res2)
    }

    fn size(&self) -> usize {
        self.size
    }
}

/// Example: Range Minimum Query
struct RMQ;
impl Monoid for RMQ {
    type T = i32;

    fn id() -> i32 {
        i32::max_value()
    }

    fn op(a: &i32, b: &i32) -> i32 {
        std::cmp::min(*a, *b)
    }
}