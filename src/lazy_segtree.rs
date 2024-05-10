pub struct LazySegmentTree<M, A>
where
    M: Monoid,
    A: Action,
{
    vals: Vec<i64>,
    lazy: Vec<i64>,
    n: usize,
    pm: std::marker::PhantomData<fn() -> M>,
    pa: std::marker::PhantomData<fn() -> A>,
}

impl<M, A> LazySegmentTree<M, A>
where
    M: Monoid,
    A: Action,
{
    pub fn new(n: usize) -> Self {
        let mut x = 1;
        while n >= x {
            x *= 2;
        }
        LazySegmentTree {
            vals: vec![M::identity(); 4 * n],
            lazy: vec![A::identity(); 4 * n],
            n: x,
            pm: std::marker::PhantomData,
            pa: std::marker::PhantomData,
        }
    }
    pub fn query(&mut self, range: std::ops::Range<usize>) -> i64 {
        self.sub_query(range.start, range.end, 0, 0, self.n)
    }
    pub fn action(&mut self, range: std::ops::Range<usize>, val: i64) {
        self.sub_action(range.start, range.end, val, 0, 0, self.n);
    }
    fn sub_query(&mut self, i: usize, j: usize, k: usize, left: usize, right: usize) -> i64 {
        self.eval(k, right - left);
        if right <= i || j <= left {
            M::identity()
        } else if i <= left && right <= j {
            self.vals[k]
        } else {
            let val_left = self.sub_query(i, j, k * 2 + 1, left, (left + right) / 2);
            let val_right = self.sub_query(i, j, k * 2 + 2, (left + right) / 2, right);
            M::apply(val_left, val_right)
        }
    }
    fn sub_action(&mut self, i: usize, j: usize, val: i64, k: usize, left: usize, right: usize) {
        self.eval(k, right - left);
        if i <= left && right <= j {
            self.lazy[k] = A::apply(self.lazy[k], val);
            self.eval(k, right - left);
        } else if i < right && left < j {
            self.sub_action(i, j, val, k * 2 + 1, left, (left + right) / 2);
            self.sub_action(i, j, val, k * 2 + 2, (left + right) / 2, right);
            self.vals[k] = M::apply(self.vals[k * 2 + 1], self.vals[k * 2 + 2]);
        }
    }
    fn eval(&mut self, k: usize, length: usize) {
        if self.lazy[k] == A::identity() {
            return;
        }
        if k < self.n - 1 {
            self.lazy[k * 2 + 1] = A::apply(self.lazy[k * 2 + 1], self.lazy[k]);
            self.lazy[k * 2 + 2] = A::apply(self.lazy[k * 2 + 2], self.lazy[k]);
        }
        self.vals[k] = A::action(self.vals[k], M::propotional(self.lazy[k], length));
        self.lazy[k] = A::identity();
    }
    pub fn set(&mut self, x: usize, val: i64) -> &mut Self {
        self.vals[x + self.n - 1] = val;
        self
    }
    pub fn biuld(&mut self) {
        for i in (0..=self.n - 2).rev() {
            self.vals[i] = M::apply(self.vals[2 * i + 1], self.vals[2 * i + 2]);
        }
    }
}

pub trait Monoid {
    fn apply(x: i64, y: i64) -> i64;
    fn propotional(x: i64, length: usize) -> i64;
    fn identity() -> i64;
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
    fn apply(x: i64, y: i64) -> i64 {
        Self::gcd(x, y)
    }
    fn propotional(x: i64, _length: usize) -> i64 {
        x
    }
    fn identity() -> i64 {
        0
    }
}
pub struct Add;

impl Monoid for Add {
    fn apply(x: i64, y: i64) -> i64 {
        x + y
    }
    fn propotional(x: i64, length: usize) -> i64 {
        x * length as i64
    }
    fn identity() -> i64 {
        0
    }
}

pub struct Min;

impl Monoid for Min {
    fn apply(x: i64, y: i64) -> i64 {
        std::cmp::min(x, y)
    }
    fn propotional(x: i64, _length: usize) -> i64 {
        x
    }
    fn identity() -> i64 {
        1 << 60
    }
}
pub struct Max;

impl Monoid for Max {
    fn apply(x: i64, y: i64) -> i64 {
        std::cmp::max(x, y)
    }
    fn propotional(x: i64, _length: usize) -> i64 {
        x
    }
    fn identity() -> i64 {
        -(1 << 60)
    }
}
pub struct Update;

impl Monoid for Update {
    fn apply(x: i64, y: i64) -> i64 {
        if y != Self::identity() {
            y
        } else {
            x
        }
    }
    fn propotional(x: i64, _length: usize) -> i64 {
        x
    }
    fn identity() -> i64 {
        1 << 60
    }
}

pub trait Action: Monoid {
    fn action(x: i64, y: i64) -> i64;
}

impl Action for Add {
    fn action(x: i64, y: i64) -> i64 {
        x + y
    }
}

impl Action for Max {
    fn action(x: i64, y: i64) -> i64 {
        i64::max(x, y)
    }
}

impl Action for Min {
    fn action(x: i64, y: i64) -> i64 {
        i64::min(x, y)
    }
}

impl Action for Gcd {
    fn action(x: i64, y: i64) -> i64 {
        Self::gcd(x, y)
    }
}

impl Action for Update {
    fn action(x: i64, y: i64) -> i64 {
        if y != <Self as Monoid>::identity() {
            y
        } else {
            x
        }
    }
}
