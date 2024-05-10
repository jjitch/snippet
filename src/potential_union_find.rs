pub trait Monoid<Element> {
    fn apply(x: &Element, y: &Element) -> Element;
    fn identity() -> Element;
}
pub trait Group<Element>: Monoid<Element> {
    fn inv(x: &Element) -> Element;
}

#[derive(Debug)]
pub struct PotentialUnionFind<Element> {
    par: Vec<usize>,
    rank: Vec<usize>,
    ptn: Vec<Element>,
}

impl<Element> PotentialUnionFind<Element> {
    pub fn new<G: Group<Element>>(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            rank: vec![1; n],
            ptn: (0..n).map(|_| G::identity()).collect(),
        }
    }
    pub fn root<G: Group<Element>>(&mut self, i: usize) -> usize {
        if self.par[i] == i {
            i
        } else {
            let p = self.par[i];
            let r = self.root::<G>(p);
            self.ptn[i] = G::apply(&self.ptn[p], &self.ptn[i]);
            self.par[i] = r;
            r
        }
    }
    pub fn diff<G: Group<Element>>(&mut self, i: usize, j: usize) -> Element {
        let p = self.root::<G>(i);
        let q = self.root::<G>(j);
        G::apply(&self.ptn[q], &(G::inv(&self.ptn[p])))
    }
    pub fn merge<G: Group<Element>>(&mut self, i: usize, j: usize, x: &Element) {}
}
