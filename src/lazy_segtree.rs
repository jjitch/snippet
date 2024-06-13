pub struct LazySegmentTree<M: Monoid, A: Act> {
    vals: Vec<M::Element>,
    lazy: Vec<A::Element>,
    n: usize,
}

impl<M, A> std::fmt::Debug for LazySegmentTree<M, A>
where
    M: Monoid,
    M::Element: std::fmt::Debug,
    A: Act,
    A::Element: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use itertools::Itertools;
        writeln!(f, "LazySegmentTree n={}", &self.n)?;
        let (mut l, mut w) = (0, 1);
        while w <= self.n {
            writeln!(
                f,
                "[{}]",
                (l..l + w)
                    .map(|i| format!("{:?}({:?})", &self.vals[i], &self.lazy[i]))
                    .join(", ")
            )?;
            l <<= 1;
            l += 1;
            w <<= 1;
        }
        Ok(())
    }
}

impl<M, A> LazySegmentTree<M, A>
where
    M: Monoid,
    A: Act<Target = M::Element>,
{
    pub fn new(n: usize) -> Self {
        let mut x = 1;
        while n > x {
            x *= 2;
        }
        LazySegmentTree {
            vals: (0..4 * n).map(|_| M::identity()).collect(),
            lazy: (0..4 * n).map(|_| A::identity()).collect(),
            n: x,
        }
    }
    pub fn operate(&mut self, range: std::ops::Range<usize>) -> M::Element {
        self.sub_operate(range.start, range.end, 0, 0, self.n)
    }
    fn sub_operate(
        &mut self,
        i: usize,
        j: usize,
        k: usize,
        left: usize,
        right: usize,
    ) -> M::Element {
        self.eval(k, right - left);
        if right <= i || j <= left {
            M::identity()
        } else if i <= left && right <= j {
            M::operate(&self.vals[k], &M::identity())
        } else {
            let val_left = self.sub_operate(i, j, k * 2 + 1, left, (left + right) / 2);
            let val_right = self.sub_operate(i, j, k * 2 + 2, (left + right) / 2, right);
            M::operate(&val_left, &val_right)
        }
    }
    pub fn act(&mut self, range: std::ops::Range<usize>, val: A::Element) {
        self.sub_action(range.start, range.end, &val, 0, 0, self.n);
    }
    fn sub_action(
        &mut self,
        i: usize,
        j: usize,
        val: &A::Element,
        k: usize,
        left: usize,
        right: usize,
    ) {
        self.eval(k, right - left);
        if i <= left && right <= j {
            self.lazy[k] = A::operate(&self.lazy[k], val);
            self.eval(k, right - left);
        } else if i < right && left < j {
            self.sub_action(i, j, val, k * 2 + 1, left, (left + right) / 2);
            self.sub_action(i, j, val, k * 2 + 2, (left + right) / 2, right);
            self.vals[k] = M::operate(&self.vals[k * 2 + 1], &self.vals[k * 2 + 2]);
        }
    }
    fn eval(&mut self, k: usize, length: usize) {
        if k < self.n - 1 {
            self.lazy[k * 2 + 1] = A::operate(&self.lazy[k * 2 + 1], &self.lazy[k]);
            self.lazy[k * 2 + 2] = A::operate(&self.lazy[k * 2 + 2], &self.lazy[k]);
        }
        self.vals[k] = A::act(&self.vals[k], &A::proportional(&self.lazy[k], length));
        self.lazy[k] = A::identity();
    }
    pub fn eval_all(&mut self) {
        for i in 0..2 * self.n - 1 {
            self.eval(
                i,
                self.n >> ((0usize.leading_zeros() - (i + 1).leading_zeros()) as usize - 1),
            );
        }
    }
    pub fn set(&mut self, x: usize, val: M::Element) -> &mut Self {
        self.vals[x + self.n - 1] = val;
        self
    }
    pub fn biuld(&mut self) {
        for i in (0..=self.n - 2).rev() {
            self.vals[i] = M::operate(&self.vals[2 * i + 1], &self.vals[2 * i + 2]);
        }
    }
}

impl<M: Monoid, A: Act<Target = M::Element>> std::iter::FromIterator<M::Element>
    for LazySegmentTree<M, A>
{
    fn from_iter<T: IntoIterator<Item = M::Element>>(iter: T) -> Self {
        let v = iter.into_iter().collect::<Vec<_>>();
        let n = v.len();
        let mut sg = LazySegmentTree::<M, A>::new(n);
        for (i, e) in v.into_iter().enumerate() {
            sg.vals[i + sg.n - 1] = e;
        }
        for i in (0..sg.n - 1).rev() {
            sg.vals[i] = M::operate(&sg.vals[2 * i + 1], &sg.vals[2 * i + 1]);
        }
        sg
    }
}

use super::algebraic::*;
