use std::collections::HashMap;

pub struct UnionFind<T: std::cmp::Eq + std::hash::Hash> {
    size: usize,
    root: Vec<usize>,
    tree_size: Vec<usize>,
    index: HashMap<T, usize>,
}

impl<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone> UnionFind<T> {
    pub fn new() -> UnionFind<T> {
        UnionFind {
            size: 0,
            root: vec![],
            tree_size: vec![],
            index: HashMap::new(),
        }
    }

    pub fn join(&mut self, a: &T, b: &T) {
        let aroot = self.get_root(a);
        let broot = self.get_root(b);
        if aroot == broot {
            return;
        }
        if self.tree_size[aroot] > self.tree_size[broot] {
            self.root[broot] = aroot;
            self.tree_size[aroot] += self.tree_size[broot];
        } else {
            self.root[aroot] = broot;
            self.tree_size[broot] += self.tree_size[aroot];
        }
    }

    pub fn connected(&mut self, a: &T, b: &T) -> bool {
        self.get_root(a) == self.get_root(b)
    }

    fn get_root(&mut self, a: &T) -> usize {
        let r_opt = self.index.get(a);
        if r_opt.is_none() {
            self.index.insert((*a).clone(), self.size);
            self.root.push(self.size);
            self.tree_size.push(1);
            self.size += 1;
            return self.size - 1;
        }
        let mut r = *r_opt.unwrap();
        while self.root[r] != r {
            r = self.root[r];
        }
        r
    }
}

