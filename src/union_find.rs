#[derive(Debug, Clone, Copy)]
pub enum Node {
    Parent(usize),
    Size(usize),
}

#[derive(Debug)]
pub struct UnionFind {
    pub comps: Vec<Node>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            comps: vec![Node::Size(1); n],
        }
    }
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            false
        } else {
            let sx = self.size(x);
            let sy = self.size(y);
            self.comps[rx] = Node::Size(sx + sy);
            self.comps[ry] = Node::Parent(rx);
            true
        }
    }
    fn root(&mut self, x: usize) -> usize {
        match self.comps[x] {
            Node::Parent(p) => {
                let r = self.root(p);
                self.comps[x] = Node::Parent(r);
                r
            }
            Node::Size(_) => x,
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        let px = self.root(x);
        let py = self.root(y);
        px == py
    }
    pub fn size(&mut self, x: usize) -> usize {
        match self.comps[x] {
            Node::Parent(p) => {
                let p = self.root(p);
                self.size(p)
            }
            Node::Size(s) => s,
        }
    }
    pub fn leaders(&self) -> impl Iterator<Item = usize> + '_ {
        self.comps.iter().enumerate().filter_map(|(i, c)| match c {
            Node::Parent(_) => None,
            Node::Size(_) => Some(i),
        })
    }
}
