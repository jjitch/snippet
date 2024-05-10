pub fn max_flow(g: &Graph, s: usize, t: usize) -> i64 {
    let n = g.len();
    let mut used = vec![false; n];
    let mut flow = 0;
    loop {
        for v in &mut used {
            *v = false;
        }
        let f = dfs(g, &mut used, s, t, PosInf);
        match f {
            Finite(i) if i == 0 => {
                break flow;
            }
            Finite(i) => flow += i,
            PosInf => unreachable!(),
            NegInf => unreachable!(),
        }
    }
}

fn dfs(g: &Graph, used: &mut Vec<bool>, s: usize, t: usize, f: InfInt) -> InfInt {
    if s == t {
        return f;
    }
    used[s] = true;
    for e in &g[s] {
        if !used[e.borrow().to] && e.borrow().cap > 0 {
            let f = dfs(g, used, e.borrow().to, t, f.min(e.borrow().cap.into()));
            if let Finite(i) = f {
                if i > 0 {
                    e.borrow_mut().cap -= i;
                    g.rev_edge(e).borrow_mut().cap += i;
                    return Finite(i);
                }
            }
        }
    }
    Finite(0)
}

#[derive(Debug, Clone)]
pub struct Edge {
    rev: usize,
    to: usize,
    cap: i64,
}

use std::cell::RefCell;
pub struct Graph(Vec<Vec<RefCell<Edge>>>);

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph(vec![vec![]; n])
    }
    fn rev_edge(&self, edge: &RefCell<Edge>) -> &RefCell<Edge> {
        &self[edge.borrow().to][edge.borrow().rev]
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let n = self[to].len();
        self[from].push(RefCell::new(Edge { rev: n, to, cap }));
        let n = self[from].len() - 1;
        self[to].push(RefCell::new(Edge {
            rev: n,
            to: from,
            cap: 0,
        }));
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl std::ops::Index<usize> for Graph {
    type Output = Vec<RefCell<Edge>>;
    fn index(&self, index: usize) -> &Self::Output {
        &(self.0[index])
    }
}

impl std::ops::IndexMut<usize> for Graph {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, AddAssign};
use InfInt::*;
#[derive(Debug, Clone, Copy)]
pub enum InfInt {
    Finite(i64),
    PosInf,
    NegInf,
}

macro_rules! impl_finite_add {
    ($self:expr, $rhs:expr) => {
        match ($self, $rhs) {
            (Finite(i), Finite(j)) => Finite(i + j),
            (PosInf, PosInf) => PosInf,
            (Finite(_), PosInf) => PosInf,
            (PosInf, Finite(_)) => PosInf,
            (NegInf, NegInf) => NegInf,
            (Finite(_), NegInf) => NegInf,
            (NegInf, Finite(_)) => NegInf,
            (_, _) => unreachable!(),
        }
    };
}

impl Add for InfInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        impl_finite_add!(self, rhs)
    }
}

impl AddAssign for InfInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = impl_finite_add!(*self, rhs);
    }
}

impl PartialEq for InfInt {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Finite(i), Finite(j)) => i.eq(&j),
            (PosInf, PosInf) => true,
            (NegInf, NegInf) => true,
            (_, _) => false,
        }
    }
}

impl Eq for InfInt {}

impl PartialOrd for InfInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (Finite(i), Finite(j)) => i.partial_cmp(&j),
            (PosInf, Finite(_)) => Some(Ordering::Greater),
            (PosInf, NegInf) => Some(Ordering::Greater),
            (Finite(_), NegInf) => Some(Ordering::Greater),
            (_, _) => Some(Ordering::Less),
        }
    }
}

impl Ord for InfInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (*self, *other) {
            (Finite(i), Finite(j)) => i.cmp(&j),
            (PosInf, Finite(_)) => Ordering::Greater,
            (PosInf, NegInf) => Ordering::Greater,
            (Finite(_), NegInf) => Ordering::Greater,
            (_, _) => Ordering::Less,
        }
    }
}

impl From<i64> for InfInt {
    fn from(i: i64) -> Self {
        Finite(i)
    }
}

impl std::str::FromStr for InfInt {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().into()
    }
}
