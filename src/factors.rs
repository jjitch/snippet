pub struct Factors {
    i: i64,
    n: i64,
    v: Vec<i64>,
}

impl std::iter::Iterator for Factors {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        while self.i * self.i < self.n {
            self.i += 1;
            if self.n % self.i == 0 {
                if self.i * self.i != self.n {
                    self.v.push(self.n / self.i);
                }
                return Some(self.i);
            }
        }
        self.v.pop()
    }
}

pub fn factors(x: i64) -> Factors {
    Factors {
        i: 0,
        n: x,
        v: vec![],
    }
}
