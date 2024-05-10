#[cfg(test)]
mod indexer_test {
    use competitive::indexer::*;
    #[test]
    fn test1() {
        let indexer = Indexer::new((10, 10), &ADJACENT);
        let mut next = indexer.generate((9, 9));
        assert_eq!(next.next(), Some(Position(8, 9)));
        assert_eq!(next.next(), Some(Position(9, 8)));
        assert_eq!(next.next(), None);
    }
}
