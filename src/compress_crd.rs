#[derive(Debug)]
pub struct CC<T: Ord> {
    m_i2c: Vec<T>,
    m_c2i: std::collections::BTreeMap<T, usize>,
}

impl<T: Ord + Clone> CC<T> {
    pub fn i2c(&self, i: usize) -> T {
        self.m_i2c[i].clone()
    }
    pub fn c2i<C>(&self, c: &C) -> usize
    where
        C: ?Sized + Ord,
        T: std::borrow::Borrow<C>,
    {
        *self.m_c2i.get(c).unwrap()
    }
    pub fn len(&self) -> usize {
        self.m_c2i.len()
    }
}

impl<Coord: Ord + Clone> std::iter::FromIterator<Coord> for CC<Coord> {
    fn from_iter<T: IntoIterator<Item = Coord>>(iter: T) -> Self {
        let uncomp: Vec<_> = iter
            .into_iter()
            .collect::<std::collections::BTreeSet<Coord>>()
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
}
