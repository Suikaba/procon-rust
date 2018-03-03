enum SkewHeap<T: Ord + Clone> {
    Empty,
    Node(T, Box<SkewHeap<T>>, Box<SkewHeap<T>>), // todo: need length?
}

impl<T: Ord + Clone> Clone for SkewHeap<T> {
    fn clone(&self) -> SkewHeap<T> {
        match *self {
            SkewHeap::Empty => SkewHeap::Empty,
            SkewHeap::Node(ref v, ref l, ref r) => SkewHeap::Node((*v).clone(), (*l).clone(), (*r).clone()),
        }
    }
}

impl<T: Ord + Clone> SkewHeap<T> {
    fn push(self, val: T) -> SkewHeap<T> {
        self.meld(SkewHeap::Node(val, Box::new(SkewHeap::Empty), Box::new(SkewHeap::Empty)))
    }

    fn meld(self, other: SkewHeap<T>) -> SkewHeap<T> {
        match (self, other) {
            (lhs, SkewHeap::Empty) => lhs,
            (SkewHeap::Empty, rhs) => rhs,
            (SkewHeap::Node(v1, l1, r1), SkewHeap::Node(v2, l2, r2)) => {
                let ((v, l, r), other) = if v1 < v2 {
                    ((v1, l1, r1), SkewHeap::Node(v2, l2, r2))
                } else {
                    ((v2, l2, r2), SkewHeap::Node(v1, l1, r1))
                };
                SkewHeap::Node(v, Box::new(SkewHeap::meld(*r, other)), l)
            }
        }
    }

    fn pop(self) -> (Option<T>, SkewHeap<T>) {
        match self {
            SkewHeap::Empty => (None, SkewHeap::Empty),
            SkewHeap::Node(v, l, r) => (Some(v), SkewHeap::meld(*l, *r))
        }
    }
}