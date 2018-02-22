struct ConvexHullTrick {
    a: VecDeque<i64>,
    b: VecDeque<i64>
}

#[allow(dead_code)]
impl ConvexHullTrick {
    fn new() -> ConvexHullTrick {
        ConvexHullTrick {
            a: VecDeque::new(),
            b: VecDeque::new()
        }
    }

    fn add_f(&mut self, aa: i64, bb: i64) {
        while self.a.len() >= 2 && self.check(self.a.len() - 2, self.a.len() - 1, aa, bb) {
            self.a.pop_back();
            self.b.pop_back();
        }
        self.a.push_back(aa);
        self.b.push_back(bb);
    }

    fn eval(&self, idx: usize, x: i64) -> i64 {
        self.a[idx] * x + self.b[idx]
    }

    // minimum query
    fn query(&mut self, x: i64) -> i64 {
        if self.a.len() == 0 {
            i64::max_value()
        } else {
            while self.a.len() >= 2 && self.eval(0, x) >= self.eval(1, x) {
                self.a.pop_front();
                self.b.pop_front();
            }
            self.eval(0, x)
        }
    }

    fn check(&self, f1: usize, f2: usize, aa: i64, bb: i64) -> bool {
        let (ref a, ref b) = (&self.a, &self.b);
        (a[f2] - a[f1]) * (bb - b[f2]) >= (b[f2] - b[f1]) * (aa - a[f2])
    }
}