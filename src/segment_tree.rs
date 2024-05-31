pub struct SegmentTree<T, M: Monoid<T>> {
    table: Vec<T>,
    n: usize,
    phantom: std::marker::PhantomData<M>,
}

impl<T, M: Monoid<T>> SegmentTree<T, M> {
    pub fn new(n: usize) -> Self {
        let mut x = 1;
        while n >= x {
            x *= 2;
        }
        Self {
            table: (0..4 * n).map(|_| M::identity()).collect(),
            n: x,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn get(&self, range: std::ops::Range<usize>) -> T {
        self.sub_query(range.start, range.end, 0, 0, self.n)
    }
    fn sub_query(&self, i: usize, j: usize, indicator: usize, left: usize, right: usize) -> T {
        if right <= i || j <= left {
            M::identity()
        } else if i <= left && right <= j {
            M::operate(&M::identity(), &self.table[indicator])
        } else {
            let val_left = self.sub_query(i, j, indicator * 2 + 1, left, (left + right) / 2);
            let val_right = self.sub_query(i, j, indicator * 2 + 2, (left + right) / 2, right);
            M::operate(&val_left, &val_right)
        }
    }
    fn propagate(&mut self, mut i: usize) {
        while i > 0 {
            i = (i - 1) / 2;
            self.table[i] = M::operate(&self.table[2 * i + 1], &self.table[2 * i + 2]);
        }
    }
    pub fn operate_asssign(&mut self, i: usize, val: T) {
        let i = i + self.n - 1;
        self.table[i] = M::operate(&self.table[i], &val);
        self.propagate(i);
    }
    pub fn asssign(&mut self, i: usize, val: T) {
        let i = i + self.n - 1;
        self.table[i] = val;
        self.propagate(i);
    }
}

impl<T: std::fmt::Debug, M: Monoid<T>> std::fmt::Debug for SegmentTree<T, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.table)
    }
}

pub struct SegmentTreeBuilder<T, M: Monoid<T>> {
    table: Vec<T>,
    n: usize,
    phantom: std::marker::PhantomData<M>,
}

impl<T, M: Monoid<T>> SegmentTreeBuilder<T, M> {
    pub fn new(n: usize) -> Self {
        let mut x = 1;
        while n >= x {
            x *= 2;
        }
        Self {
            table: (0..4 * n).map(|_| M::identity()).collect(),
            n: x,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn set(mut self, i: usize, val: &T) -> Self {
        self.table[i + self.n - 1] = M::operate(&self.table[i + self.n - 1], &val);
        self
    }
    pub fn biuld(mut self) -> SegmentTree<T, M> {
        for i in (0..self.n - 1).rev() {
            self.table[i] = M::operate(&self.table[2 * i + 1], &self.table[2 * i + 2]);
        }
        SegmentTree::<T, M> {
            table: self.table,
            n: self.n,
            phantom: std::marker::PhantomData,
        }
    }
}

use super::algebraic::*;
