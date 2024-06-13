mod lazy_segtree_test {
    use snippet::algebraic::*;
    use snippet::lazy_segtree::*;

    #[test]
    fn min_update() {
        let mut lst = LazySegmentTree::<Min, Assign<MinElm>>::new(8);
        lst.act(0..8, Some(MinElm::Finite(0))); // 0 0 0 0 0 0 0 0
        lst.act(0..2, Some(MinElm::Finite(1))); // 1 1 0 0 0 0 0 0
        lst.act(1..3, Some(MinElm::Finite(3))); // 1 3 3 0 0 0 0 0
        lst.act(2..3, Some(MinElm::Finite(2))); // 1 3 2 0 0 0 0 0
        assert_eq!(lst.operate(0..3).unwrap(), 1);
        assert_eq!(lst.operate(1..3).unwrap(), 2);
        lst.act(2..6, Some(MinElm::Finite(1))); // 1 3 1 1 1 1 0 0
        lst.act(5..8, Some(MinElm::Finite(6))); // 1 3 1 1 1 6 6 6
        assert_eq!(lst.operate(0..8).unwrap(), 1);
        assert_eq!(lst.operate(4..8).unwrap(), 1);
        assert_eq!(lst.operate(8..8), Min::identity());
    }
    #[test]
    fn add_add() {
        let mut lst = LazySegmentTree::<Add, Add>::new(8);
        lst.act(0..2, 1); // 1 1 0 0 0 0 0 0
        lst.act(1..3, 3); // 1 4 3 0 0 0 0 0
        lst.act(2..3, 2); // 1 4 5 0 0 0 0 0
        assert_eq!(lst.operate(0..3), 10);
        assert_eq!(lst.operate(1..3), 9);
        lst.act(2..6, 1); // 1 4 6 1 1 1 0 0
        lst.act(5..8, 6); // 1 4 6 1 1 7 6 6
        assert_eq!(lst.operate(0..8), 32);
        assert_eq!(lst.operate(4..8), 20);
        assert_eq!(lst.operate(8..8), 0);
    }

    #[test]
    fn min_add() {
        let mut lst = vec![0; 8]
            .into_iter()
            .map(|i| MinElm::Finite(i))
            .collect::<LazySegmentTree<Min, GeneralAdd<MinElm>>>();
        lst.act(0..2, 1.into()); // 1 1 0 0 0 0 0 0
        lst.act(1..3, 3.into()); // 1 4 3 0 0 0 0 0
        lst.act(2..3, 2.into()); // 1 4 5 0 0 0 0 0
        assert_eq!(lst.operate(0..3).unwrap(), 1);
        assert_eq!(lst.operate(1..3).unwrap(), 4);
        lst.act(2..6, 1.into()); // 1 4 6 1 1 1 0 0
        lst.act(5..8, 6.into()); // 1 4 6 1 1 7 6 6
        assert_eq!(lst.operate(0..8).unwrap(), 1.into());
        assert_eq!(lst.operate(4..8).unwrap(), 1.into());
        assert_eq!(lst.operate(8..8), Min::identity());
    }
}
