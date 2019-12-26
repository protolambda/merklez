use std::sync::{Mutex, Arc};
use crate::tree::{Root, ZERO, Hasher, Node, Pair};

pub struct Commit<T: Node> {
    // TODO; once-mutex to do concurrent caching, then lock-free reads
    value: Mutex<Root>,
    left: T,
    right: T,
}

impl Node for Commit<_> {
    fn merkle_root(&self, h: Hasher) -> Root {
        let mut value = self.value.lock().unwrap();
        if *value == ZERO {
            *value = h(self.left.merkle_root(), self.right.merkle_root());
            *value
        } else{
            *value
        }
    }
}

impl<T: Node> Pair<T> for Commit<T> {
    fn pair(l: T, r: T) -> &Self {
        &Commit {
            value: Mutex::new(ZERO),
            left: l,
            right: r
        }
    }
}
