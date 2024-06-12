pub struct SegmentTree<M: Monoid> {
    table: Vec<M::Element>,
    n: usize,
}

impl<M: Monoid> SegmentTree<M> {
    pub fn new(n: usize) -> Self {
        let mut x = 1;
        while n >= x {
            x *= 2;
        }
        Self {
            table: (0..4 * n).map(|_| M::identity()).collect(),
            n: x,
        }
    }
    pub fn get(&self, range: std::ops::Range<usize>) -> M::Element {
        self.sub_query(range.start, range.end, 0, 0, self.n)
    }
    fn sub_query(
        &self,
        i: usize,
        j: usize,
        indicator: usize,
        left: usize,
        right: usize,
    ) -> M::Element {
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
    pub fn operate_asssign(&mut self, i: usize, val: M::Element) {
        let i = i + self.n - 1;
        self.table[i] = M::operate(&self.table[i], &val);
        self.propagate(i);
    }
    pub fn asssign(&mut self, i: usize, val: M::Element) {
        let i = i + self.n - 1;
        self.table[i] = val;
        self.propagate(i);
    }
}

impl<T: std::fmt::Debug, M: Monoid<Element = T>> std::fmt::Debug for SegmentTree<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.table)
    }
}

impl<M: Monoid> FromIterator<M::Element> for SegmentTree<M> {
    fn from_iter<T: IntoIterator<Item = M::Element>>(iter: T) -> Self {
        let v = iter.into_iter().collect::<Vec<_>>();
        let n = v.len();
        let mut sg = SegmentTree::<M>::new(n);
        for i in 0..n {
            sg.table[i + sg.n - 1] = M::operate(&sg.table[i + sg.n - 1], &v[i]);
        }
        for i in (0..sg.n - 1).rev() {
            sg.table[i] = M::operate(&sg.table[2 * i + 1], &sg.table[2 * i + 2]);
        }
        sg
    }
}

use super::algebraic::*;
