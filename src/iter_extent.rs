impl<T: ?Sized> IterExtent for T where T: Iterator {}
pub trait IterExtent
where
    Self: Iterator,
{
    fn counts(self) -> std::collections::HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + std::hash::Hash,
    {
        let mut h = std::collections::HashMap::new();
        self.for_each(|v| *h.entry(v).or_default() += 1);
        h
    }
}
