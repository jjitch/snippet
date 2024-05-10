/// Fenwick Tree
pub struct FenwickTree {
    pub item: Vec<i64>,
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        FenwickTree {
            item: vec![0i64; n + 1],
        }
    }
    pub fn from_vec(v: &[i64]) -> Self {
        let n = v.len() + 1;
        let mut a = vec![0i64; n];
        for i in 1..n {
            a[i] += v[i - 1];
            let j = i + (i & i.wrapping_neg());
            if j < n {
                a[j] += a[i];
            }
        }
        FenwickTree { item: a }
    }
    pub fn add(&mut self, i: usize, val: i64) {
        let mut i = i + 1;
        while i <= self.item.len() - 1 {
            self.item[i] += val;
            i += i & i.wrapping_neg();
        }
    }
    pub fn sum(&self, i: usize, j: usize) -> i64 {
        self.prefix_sum(j) - self.prefix_sum(i)
    }
    fn prefix_sum(&self, mut i: usize) -> i64 {
        let mut s = 0;
        while i > 0 {
            s += self.item[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}

impl std::fmt::Debug for FenwickTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.item.len() - 1;
        let mut v = vec![];
        for i in 0..n {
            v.push(self.sum(i, i + 1));
        }
        write!(f, "{:?}", v)
    }
}
