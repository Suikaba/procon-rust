/// Kruskal algorithm (Minimum Spanning Tree)
/// param V: number of nodes
/// param edges: edges of graph
#[allow(dead_code)]
fn kruskal(vsize: usize, edges: &mut Vec<WeightedEdge>) -> Vec<WeightedEdge> {
    let mut res: Vec<WeightedEdge> = Vec::new();
    let mut uf: UnionFind = UnionFind::new(vsize);
    edges.sort_by(|e1, e2| e1.cmp(e2));
    for e in edges {
        if !uf.same(e.from, e.to) {
            uf.unite(e.from, e.to);
            res.push(WeightedEdge::new(e.from, e.to, e.weight));
        }
    }
    res
}