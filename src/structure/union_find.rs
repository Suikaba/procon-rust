//use std::vec::Vec;

struct UnionFind {
    par: Vec<i32>,
}

impl UnionFind {
    #[allow(dead_code)]
    fn new(n: usize) -> UnionFind {
        UnionFind {
            par: vec![-1; n],
        }
    }

    #[allow(dead_code)]
    fn root(&mut self, x: usize) -> usize {
        if self.par[x] < 0 {
            x
        } else {
            let p = self.par[x] as usize;
            self.par[x] = self.root(p) as i32;
            self.par[x] as usize
        }
    }

    #[allow(dead_code)]
    fn unite(&mut self, x: usize, y: usize) -> bool {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            false
        } else {
            if self.size(x) < self.size(y) {
                self.par[y] += self.par[x];
                self.par[x] = y as i32; // usize <-> i32
            } else {
                self.par[x] += self.par[y];
                self.par[y] = x as i32;
            }
            true
        }
    }

    #[allow(dead_code)]
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    #[allow(dead_code)]
    fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        (-self.par[r]) as usize
    }
}