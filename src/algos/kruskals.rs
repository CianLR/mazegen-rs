use std::collections::HashMap;

use rand::prelude::*;

use crate::algos::algo;
use crate::maze::Maze;

struct UnionFind<T: std::cmp::Eq + std::hash::Hash> {
    size: usize,
    root: Vec<usize>,
    tree_size: Vec<usize>,
    index: HashMap<T, usize>,
}

impl<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone> UnionFind<T> {
    fn new() -> UnionFind<T> {
        UnionFind {
            size: 0,
            root: vec![],
            tree_size: vec![],
            index: HashMap::new(),
        }
    }

    fn join(&mut self, a: &T, b: &T) {
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

    fn connected(&mut self, a: &T, b: &T) -> bool {
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

pub struct KruskalsAlgo;

impl KruskalsAlgo {
    fn kruskals(maze: &mut Maze) -> Result<(), String> {
        let mut uf = UnionFind::new();
        let mut walls = vec![];
        let size = maze.get_size();
        for x in 0..size {
            for y in 0..size {
                for (x2, y2, d) in maze.get_adjacent(x, y) {
                    walls.push((x, y, x2, y2, d));
                }
            }
        }
        walls.shuffle(&mut rand::thread_rng());
        while !walls.is_empty() {
            let (x, y, x2, y2, d) = walls.pop().unwrap();
            if uf.connected(&(x, y), &(x2, y2)) {
                continue;
            }
            uf.join(&(x, y), &(x2, y2));
            maze.rm_wall(x, y, d);
            maze.rm_wall(x2, y2, d.opposite());
        }
        Ok(())
    }
}

impl algo::MazeAlgo for KruskalsAlgo {
    fn generate(maze: &mut Maze) -> Result<(), String> {
        maze.fill_walls();
        KruskalsAlgo::kruskals(maze)
    }
}
