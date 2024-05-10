mod lazy_segtree_test {
    use competitive::lazy_segtree::*;
    #[test]
    fn add_update() {
        let mut lst = LazySegmentTree::<Add, Update>::new(8);
        lst.action(0..2, 1); // 1 1 0 0 0 0 0 0
        lst.action(1..3, 3); // 1 3 3 0 0 0 0 0
        lst.action(2..3, 2); // 1 3 2 0 0 0 0 0
        assert_eq!(lst.query(0..3), 6);
        assert_eq!(lst.query(1..3), 5);
        lst.action(2..6, 1); // 1 3 1 1 1 1 0 0
        lst.action(5..8, 6); // 1 3 1 1 1 6 6 6
        assert_eq!(lst.query(0..8), 25);
        assert_eq!(lst.query(4..8), 19);
        assert_eq!(lst.query(8..8), 0);
    }
    #[test]
    fn min_update() {
        let mut lst = LazySegmentTree::<Min, Update>::new(8);
        for i in 0..8 {
            lst.set(i, 0);
        }
        lst.biuld();
        lst.action(0..2, 1); // 1 1 0 0 0 0 0 0
        lst.action(1..3, 3); // 1 3 3 0 0 0 0 0
        lst.action(2..3, 2); // 1 3 2 0 0 0 0 0
        assert_eq!(lst.query(0..3), 1);
        assert_eq!(lst.query(1..3), 2);
        lst.action(2..6, 1); // 1 3 1 1 1 1 0 0
        lst.action(5..8, 6); // 1 3 1 1 1 6 6 6
        assert_eq!(lst.query(0..8), 1);
        assert_eq!(lst.query(4..8), 1);
        assert_eq!(lst.query(8..8), Min::identity());
    }
    #[test]
    fn add_add() {
        let mut lst = LazySegmentTree::<Add, Add>::new(8);
        lst.action(0..2, 1); // 1 1 0 0 0 0 0 0
        lst.action(1..3, 3); // 1 4 3 0 0 0 0 0
        lst.action(2..3, 2); // 1 4 5 0 0 0 0 0
        assert_eq!(lst.query(0..3), 10);
        assert_eq!(lst.query(1..3), 9);
        lst.action(2..6, 1); // 1 4 6 1 1 1 0 0
        lst.action(5..8, 6); // 1 4 6 1 1 7 6 6
        assert_eq!(lst.query(0..8), 32);
        assert_eq!(lst.query(4..8), 20);
        assert_eq!(lst.query(8..8), 0);
    }
    #[test]
    fn min_add() {
        let mut lst = LazySegmentTree::<Min, Add>::new(8);
        for i in 0..8 {
            lst.set(i, 0);
        }
        lst.biuld();
        lst.action(0..2, 1); // 1 1 0 0 0 0 0 0
        lst.action(1..3, 3); // 1 4 3 0 0 0 0 0
        lst.action(2..3, 2); // 1 4 5 0 0 0 0 0
        assert_eq!(lst.query(0..3), 1);
        assert_eq!(lst.query(1..3), 4);
        lst.action(2..6, 1); // 1 4 6 1 1 1 0 0
        lst.action(5..8, 6); // 1 4 6 1 1 7 6 6
        assert_eq!(lst.query(0..8), 1);
        assert_eq!(lst.query(4..8), 1);
        assert_eq!(lst.query(8..8), Min::identity());
    }
}
