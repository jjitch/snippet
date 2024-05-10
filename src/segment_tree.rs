pub trait Monoid {
    type Element: Copy + std::fmt::Debug;
    fn apply(x: Self::Element, y: Self::Element) -> Self::Element;
    fn identity() -> Self::Element;
}

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
            table: vec![M::identity(); 4 * n],
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
            self.table[indicator]
        } else {
            let val_left = self.sub_query(i, j, indicator * 2 + 1, left, (left + right) / 2);
            let val_right = self.sub_query(i, j, indicator * 2 + 2, (left + right) / 2, right);
            M::apply(val_left, val_right)
        }
    }
    pub fn set(&mut self, i: usize, val: M::Element) {
        let mut i = i + self.n - 1;
        self.table[i] = M::apply(self.table[i], val);
        while i > 0 {
            i = (i - 1) / 2;
            self.table[i] = M::apply(self.table[2 * i + 1], self.table[2 * i + 2]);
        }
    }
}

impl<M: Monoid> std::fmt::Debug for SegmentTree<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.table)
    }
}

pub struct SegmentTreeBuilder<M>
where
    M: Monoid,
{
    table: Vec<M::Element>,
    n: usize,
}

impl<M: Monoid> SegmentTreeBuilder<M> {
    pub fn new(n: usize) -> Self {
        let mut x = 1;
        while n >= x {
            x *= 2;
        }
        Self {
            table: vec![M::identity(); 4 * n],
            n: x,
        }
    }
    pub fn set(mut self, i: usize, val: M::Element) -> Self {
        self.table[i + self.n - 1] = M::apply(self.table[i + self.n - 1], val);
        self
    }
    pub fn biuld(mut self) -> SegmentTree<M> {
        for i in (0..self.n - 1).rev() {
            self.table[i] = M::apply(self.table[2 * i + 1], self.table[2 * i + 2]);
        }
        SegmentTree {
            table: self.table,
            n: self.n,
        }
    }
}

pub struct Gcd;
impl Gcd {
    fn gcd(x: i64, y: i64) -> i64 {
        if y == 0 {
            x
        } else {
            Self::gcd(y, x % y)
        }
    }
}
impl Monoid for Gcd {
    type Element = i64;
    fn apply(x: Self::Element, y: Self::Element) -> Self::Element {
        Self::gcd(x, y)
    }
    fn identity() -> Self::Element {
        0
    }
}
pub struct Add;

impl Monoid for Add {
    type Element = i64;
    fn apply(x: Self::Element, y: Self::Element) -> Self::Element {
        x + y
    }
    fn identity() -> Self::Element {
        0
    }
}

pub struct Min;

impl Monoid for Min {
    type Element = i64;
    fn apply(x: Self::Element, y: Self::Element) -> Self::Element {
        std::cmp::min(x, y)
    }
    fn identity() -> Self::Element {
        1 << 60
    }
}
pub struct Max;

impl Monoid for Max {
    type Element = i64;
    fn apply(x: Self::Element, y: Self::Element) -> Self::Element {
        std::cmp::max(x, y)
    }
    fn identity() -> Self::Element {
        -(1 << 60)
    }
}
