use std::vec;

struct UnionFind {
    par: vec::Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            par: vec![-1; n],
        }
    }

    #[allow(dead_code)]
    fn root(&mut self, x: i32) -> i32 {
        let x_id = x as usize;
        if self.par[x_id] < 0 {
            x
        } else {
            let p = self.par[x_id];
            self.par[x_id] = self.root(p);
            self.par[x_id]
        }
    }

    #[allow(dead_code)]
    fn unite(&mut self, x: i32, y: i32) -> bool {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            false
        } else {
            let x_id = x as usize;
            let y_id = y as usize;
            if self.size(x) < self.size(y) {
                self.par[y_id] = self.par[y_id] + self.par[x_id];
                self.par[x_id] = y;
            } else {
                self.par[x_id] = self.par[x_id] + self.par[y_id];
                self.par[y_id] = x;
            }
            true
        }
    }

    #[allow(dead_code)]
    fn same(&mut self, x: i32, y: i32) -> bool {
        self.root(x) == self.root(y)
    }

    #[allow(dead_code)]
    fn size(&mut self, x: i32) -> i32 {
        let r = self.root(x) as usize;
        -self.par[r]
    }
}