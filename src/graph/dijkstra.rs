#[allow(dead_code)]
fn dijkstra(g: &WeightedGraph, s: usize) -> (std::vec::Vec<Option<i32>>, std::vec::Vec<Option<usize>>) {
    let mut d = vec![None; g.node_size()];
    let mut prev = vec![None; g.node_size()];
    let mut que: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    d[s] = Some(0);
    que.push((0, s));
    while let Some((cur_d, v)) = que.pop() {
        if d[v].unwrap() < -cur_d {
            continue;
        }
        for ref e in g.edges(v) {
            if d[e.to].is_none() || d[e.to].unwrap() > -cur_d + e.weight {
                d[e.to] = Some(-cur_d + e.weight);
                prev[e.to] = Some(v);
                que.push((cur_d - e.weight, e.to));
            }
        }
    }
    (d, prev)
}