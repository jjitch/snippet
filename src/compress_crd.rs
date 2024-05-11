pub struct CC<T: std::cmp::Ord> {
    m_i2c: Vec<T>,
    m_c2i: std::collections::BTreeMap<T, usize>,
}

impl<T: std::cmp::Ord + Clone> CC<T> {
    pub fn new(v: &Vec<T>) -> Self {
        let uncomp: Vec<_> = v
            .iter()
            .map(|v| v.clone())
            .collect::<std::collections::BTreeSet<T>>()
            .into_iter()
            .collect();
        let compress = uncomp
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();
        Self {
            m_i2c: uncomp,
            m_c2i: compress,
        }
    }
    pub fn i2c(&self, i: usize) -> T {
        self.m_i2c[i].clone()
    }
    pub fn c2i(&self, c: &T) -> usize {
        *self.m_c2i.get(c).unwrap()
    }
}
