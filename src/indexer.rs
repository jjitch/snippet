pub const ADJACENT: [(i64, i64); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

pub struct Indexer<'a> {
    limit: (usize, usize),
    shift: &'a [(i64, i64)],
}

impl<'a> Indexer<'a> {
    pub fn new(limit: (usize, usize), shift: &'a [(i64, i64)]) -> Self {
        Indexer { limit, shift }
    }
    pub fn generate<P: Into<Position>>(&self, pos: P) -> ShiftedIndex {
        let pos: Position = pos.into();
        ShiftedIndex {
            cursor: 0,
            pos: (pos.0 as i64, pos.1 as i64),
            limit: (self.limit.0 as i64, self.limit.1 as i64),
            shift: self.shift,
        }
    }
}

pub struct ShiftedIndex<'a> {
    cursor: usize,
    pos: (i64, i64),
    limit: (i64, i64),
    shift: &'a [(i64, i64)],
}

impl<'a> Iterator for ShiftedIndex<'a> {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        while self.cursor < self.shift.len() {
            let (i, j) = (
                self.pos.0 + self.shift[self.cursor].0,
                self.pos.1 + self.shift[self.cursor].1,
            );
            self.cursor += 1;
            if 0 <= i && i < self.limit.0 && 0 <= j && j < self.limit.1 {
                return Some(Position(i as usize, j as usize));
            }
        }
        None
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct Position(pub usize, pub usize);

use std::ops::{Index, IndexMut};
impl<T> Index<Position> for Vec<Vec<T>> {
    type Output = T;
    fn index(&self, index: Position) -> &Self::Output {
        &self[index.0][index.1]
    }
}

impl<T> IndexMut<Position> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self[index.0][index.1]
    }
}

impl From<(usize, usize)> for Position {
    fn from(t: (usize, usize)) -> Self {
        Self(t.0, t.1)
    }
}
