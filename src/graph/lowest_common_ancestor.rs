struct LowestCommonAncestor {
    //n: usize,
    log_n: usize,
    parent: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}

#[allow(dead_code)]
impl LowestCommonAncestor {
    fn new<G: Graph<V = usize>>(t: &G, root: usize) -> LowestCommonAncestor {
        let mut n = t.node_size();
        let mut log_n: usize = 0;
        while n > 0 {
            log_n += 1;
            n >>= 1;
        }
        let log_n = log_n;
        let n = t.node_size();

        fn dfs<G: Graph<V = usize>>(t: &G, v: usize, p: Option<usize>, d: usize, parent: &mut Vec<Vec<Option<usize>>>, depth: &mut Vec<usize>) {
            parent[0][v] = p;
            depth[v] = d;
            for e in t.edges(v) {
                if p.is_none() || e.to() != p.unwrap() {
                    dfs(t, e.to(), Some(v), d + 1, parent, depth);
                }
            }
        }

        let mut parent: Vec<Vec<Option<usize>>> = vec![vec![None; n]; log_n];
        let mut depth: Vec<usize> = vec![0; n];
        dfs(t, root, None, 0, &mut parent, &mut depth);
        for k in 0..log_n-1 {
            for v in 0..n {
                match parent[k][v] {
                    Some(p) => parent[k + 1][v] = parent[k][p],
                    None    => parent[k + 1][v] = None,
                }
            }
        }

        LowestCommonAncestor {
            //n: n,
            log_n: log_n,
            parent: parent,
            depth: depth
        }
    }

    fn depth(&self, v: usize) -> usize {
        self.depth[v]
    }

    fn dist(&self, u: usize, v: usize) -> usize {
        self.depth[u] + self.depth[v] - 2 * self.depth[self.query(u, v)]
    }

    fn parent(&self, u: usize, k: usize) -> Option<usize> {
        let mut res: Option<usize> = Some(u);
        for i in 0..self.log_n {
            if k & (1 << i) > 0 {
                match self.parent[i][res.unwrap()] {
                    Some(p) => res = Some(p),
                    None    => {
                        res = None;
                        break;
                    }
                }
            }
        }
        res
    }

    fn query(&self, u: usize, v: usize) -> usize {
        let (mut u, mut v) = if self.depth(u) > self.depth(v) { (v, u) } else { (u, v) };
        for k in 0..self.log_n {
            if (self.depth(v) - self.depth(u)) >> k & 1 > 0 {
                v = self.parent[k][v].unwrap();
            }
        }
        let res: usize;
        if u == v {
            res = u;
        } else {
            for k in (0..self.log_n).rev() {
                if self.parent[k][u] != self.parent[k][v] {
                    u = self.parent[k][u].unwrap();
                    v = self.parent[k][v].unwrap();
                }
            }
            res = self.parent[0][u].unwrap();
        }

        res
    }
}