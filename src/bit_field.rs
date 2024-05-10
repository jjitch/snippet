pub trait BitField {
    fn val(&self) -> usize;
    fn test(&self, i: usize) -> bool {
        self.val() >> i & 1 > 0
    }
    fn any(&self) -> bool {
        self.val() > 0
    }
    fn none(&self) -> bool {
        self.val() == 0
    }
    fn set(&self, i: usize) -> usize {
        self.val() | 1 << i
    }
    fn reset(&self, i: usize) -> usize {
        if self.test(i) {
            self.val() ^ 1 << i
        } else {
            self.val()
        }
    }
    fn flip(&self) -> usize {
        std::ops::Not::not(self.val())
    }
    fn standing_bits(&self) -> StandingBits {
        StandingBits {
            bit: self.val(),
            i: 0,
        }
    }
}

impl BitField for usize {
    fn val(&self) -> usize {
        *self
    }
}

pub struct StandingBits {
    bit: usize,
    i: usize,
}

impl Iterator for StandingBits {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while self.bit > 0 {
            let stand = self.bit & 1 > 0;
            self.bit >>= 1;
            self.i += 1;
            if stand {
                return Some(self.i - 1);
            }
        }
        None
    }
}
