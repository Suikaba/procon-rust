#[allow(dead_code)]
fn dijkstra(g: &WeightedGraph, s: usize) -> (std::vec::Vec<Option<i32>>, std::vec::Vec<Option<usize>>) {
    let mut d = vec![None; g.node_size()];
    let mut prev = vec![None; g.node_size()];
    let mut que: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
    que.push((0, s));
    while let Some((cur_d, v, pre)) = que.pop() {
        if d[v].is_some() {
            continue;
        }
        d[v] = Some(-cur_d); // reverse order
        prev[v] = Some(pre);
        for ref e in g.edges(v) {
            if d[e.to].is_none() {
                que.push((cur_d - e.weight, e.to, v));
            }
        }
    }
    (d, prev)
}