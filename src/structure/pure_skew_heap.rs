use std::rc::Rc;
enum PureSkewHeap<T: Ord + Clone> {
    Empty,
    Node(T, Rc<PureSkewHeap<T>>, Rc<PureSkewHeap<T>>), // todo: need length?
}

impl<T: Ord + Clone> Clone for PureSkewHeap<T> {
    fn clone(&self) -> PureSkewHeap<T> {
        match *self {
            PureSkewHeap::Empty => PureSkewHeap::Empty,
            PureSkewHeap::Node(ref v, ref l, ref r) => PureSkewHeap::Node((*v).clone(), (*l).clone(), (*r).clone()),
        }
    }
}

impl<T: Ord + Clone> PureSkewHeap<T> {
    fn push(&self, val: T) -> PureSkewHeap<T> {
        self.meld(PureSkewHeap::Node(val, Rc::new(PureSkewHeap::Empty), Rc::new(PureSkewHeap::Empty)))
    }

    fn meld(&self, other: PureSkewHeap<T>) -> PureSkewHeap<T> {
        match *self {
            PureSkewHeap::Empty => other.clone(),
            PureSkewHeap::Node(ref v1, ref l1, ref r1) => {
                match other {
                    PureSkewHeap::Empty => self.clone(),
                    PureSkewHeap::Node(ref v2, ref l2, ref r2) => {
                        if v1 < v2 {
                            let r = r1.clone();
                            PureSkewHeap::Node(v1.clone(), l1.clone(), Rc::new(r.meld(other.clone())))
                        } else {
                            let r = r2.clone();
                            PureSkewHeap::Node(v2.clone(), l2.clone(), Rc::new(r.meld(self.clone())))
                        }
                    }
                }
            }
        }
    }

    fn pop(&self) -> (Option<T>, PureSkewHeap<T>) {
        match *self {
            PureSkewHeap::Empty => (None, PureSkewHeap::Empty),
            PureSkewHeap::Node(ref v, ref l, ref r) => {
                (Some(v.clone()), l.meld((**r).clone()))
            }
        }
    }
}