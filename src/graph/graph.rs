trait Edge {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
}

#[allow(dead_code)]
#[derive(Eq, PartialEq, PartialOrd, Clone)]
struct WeightedEdge {
    from: usize,
    to: usize,
    weight: i32,
}

impl Edge for WeightedEdge {
    fn from(&self) -> usize {
        self.from
    }
    fn to(&self) -> usize {
        self.to
    }
}

#[allow(dead_code)]
impl WeightedEdge {
    fn new(from: usize, to: usize, weight: i32) -> WeightedEdge {
        WeightedEdge {
            from: from,
            to: to,
            weight: weight
        }
    }
}

impl Ord for WeightedEdge {
    fn cmp(&self, other: &WeightedEdge) -> Ordering {
        if self.weight > other.weight {
            Ordering::Greater
        } else if self.weight < other.weight {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

///
///
///
trait Graph<E: Edge> {
    fn edges(&self, v: usize) -> &std::vec::Vec<E>;
    fn node_size(&self) -> usize;
}

#[derive(Clone)]
struct WeightedGraph {
    adj_list: std::vec::Vec<std::vec::Vec<WeightedEdge>>,
}

#[allow(dead_code)]
impl WeightedGraph {
    fn new(n: usize) -> WeightedGraph {
        WeightedGraph {
            adj_list: vec![std::vec::Vec::<WeightedEdge>::new(); n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, w: i32) {
        self.adj_list[from].push(WeightedEdge::new(from, to, w));
    }
}

impl Graph<WeightedEdge> for WeightedGraph {
    fn edges(&self, v: usize) -> &std::vec::Vec<WeightedEdge> {
        &self.adj_list[v]
    }

    fn node_size(&self) -> usize {
        self.adj_list.len()
    }
}