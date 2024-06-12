pub struct PotentialUnionFind<G: Group> {
    parent: Vec<usize>,
    rank: Vec<usize>,
    potential: Vec<G::Element>,
}

impl<D, G> std::fmt::Debug for PotentialUnionFind<G>
where
    D: Debug,
    G: Group<Element = D>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PotentialUnionFind")
            .field("parent", &self.parent)
            .field("rank", &self.rank)
            .field("potential", &self.potential)
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Diff<T> {
    Ok(T),
    Ambiguous,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Merge {
    Done,
    Redundant,
}

impl<G: Group> PotentialUnionFind<G> {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            potential: (0..n).map(|_| G::identity()).collect(),
        }
    }
    pub fn root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let p = self.parent[i];
            let r = self.root(p);
            self.potential[i] = G::operate(&self.potential[p], &self.potential[i]);
            self.parent[i] = r;
            r
        }
    }
    pub fn diff(&mut self, i: usize, j: usize) -> Diff<G::Element> {
        let p = self.root(i);
        let q = self.root(j);
        if p == q {
            Diff::Ok(G::operate(
                &self.potential[j],
                &(G::inverse(&self.potential[i])),
            ))
        } else {
            Diff::Ambiguous
        }
    }
    pub fn merge(&mut self, i: usize, j: usize, x: &G::Element) -> Merge {
        let p = self.root(i);
        let q = self.root(j);
        if p == q {
            Merge::Redundant
        } else {
            let (p, q, i, j, x) = if self.rank[p] > self.rank[q] {
                (p, q, i, j, G::operate(&G::identity(), x))
            } else {
                (q, p, j, i, G::inverse(x))
            };
            self.rank[p] += 1;
            self.parent[q] = p;
            self.potential[q] = G::operate(
                &G::operate(&self.potential[i], &x),
                &G::inverse(&self.potential[j]),
            );
            Merge::Done
        }
    }
}

use std::fmt::Debug;

use super::algebraic::*;
