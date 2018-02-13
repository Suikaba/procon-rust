struct StronglyConnectedComponent<G: Graph + Clone> {
    g: G,
    comp: Vec<usize>,
    k: usize,
}

#[allow(dead_code)]
impl<G: Graph<V = usize> + Clone> StronglyConnectedComponent<G> {
    fn new(g: &G) -> StronglyConnectedComponent<G> {
        let mut rg: Vec<Vec<usize>> = vec![Vec::<usize>::new(); g.node_size()];
        for v in 0..g.node_size() {
            for e in g.edges(v) {
                rg[e.to()].push(e.from());
            }
        }

        fn dfs<G: Graph<V = usize>>(g: &G, v: usize, vs: &mut Vec<usize>, used: &mut Vec<bool>) {
            used[v] = true;
            for e in g.edges(v) {
                if !used[e.to()] {
                    dfs(g, e.to(), vs, used);
                }
            }
            vs.push(v);
        }
        fn rdfs(rg: &Vec<Vec<usize>>, v: usize, k: usize, comp: &mut Vec<usize>, used: &mut Vec<bool>) {
            used[v] = true;
            comp[v] = k;
            for &to in &rg[v] {
                if !used[to] {
                    rdfs(rg, to, k, comp, used);
                }
            }
        }

        let mut used: Vec<bool> = vec![false; g.node_size()];
        let mut vs: Vec<usize> = Vec::new();
        for v in 0..g.node_size() {
            if !used[v] {
                dfs(g, v, &mut vs, &mut used);
            }
        }
        used = used.into_iter().map(|_x| false).collect();
        let mut k = 0;
        let mut comp: Vec<usize> = vec![0; g.node_size()];
        for &v in vs.iter().rev() {
            if !used[v] {
                rdfs(&rg, v, k, &mut comp, &mut used);
                k += 1;
            }
        }

        StronglyConnectedComponent::<G> {
            g: g.clone(),
            comp: comp,
            k: k
        }
    }

    fn belongs(&self, v: usize) -> usize {
        self.comp[v]
    }

    fn build_graph(&self) -> Vec<Vec<usize>> {
        use std::collections::HashSet;
        let k = self.k;
        let mut s: Vec<HashSet<usize>> = vec![HashSet::<usize>::new(); k];
        let mut res: Vec<Vec<usize>> = vec![Vec::<usize>::new(); k];
        for v in 0..self.g.node_size() {
            for e in self.g.edges(v) {
                s[self.belongs(v)].insert(self.belongs(e.to()));
            }
        }
        for i in 0..k {
            for &j in &s[i] {
                if i != j {
                    res[i].push(j);
                }
            }
        }
        res
    }
}