mod segment_tree_test {
    use snippet::algebraic::*;
    use snippet::segment_tree::*;
    #[test]
    fn add_1() {
        let mut segtree = SegmentTree::<Add>::new(10);
        segtree.asssign(0, 10);
        segtree.asssign(1, 10);
        segtree.asssign(3, 10);
        assert_eq!(segtree.get(1..2), 10);
        assert_eq!(segtree.get(1..3), 10);
        assert_eq!(segtree.get(1..4), 20);
        assert_eq!(segtree.get(0..4), 30);
    }
    #[test]
    fn min_1() {
        let set_query = vec![
            MinElm::Finite(5),
            MinElm::Finite(6),
            MinElm::Finite(1),
            MinElm::Finite(365),
            MinElm::Finite(44),
            MinElm::Finite(6),
        ];
        let st = set_query.into_iter().collect::<SegmentTree<Min>>();
        println!("{:?}", st);
        assert_eq!(st.get(0..6), MinElm::Finite(1));
        assert_eq!(st.get(3..4), MinElm::Finite(365));
        assert_eq!(st.get(0..3), MinElm::Finite(1));
        assert_eq!(st.get(3..5), MinElm::Finite(44));
    }
    #[test]
    fn max_1() {
        let set_query = vec![5, 6, 1, 365, 44, 6];
        let st = set_query
            .into_iter()
            .map(|i| i.into())
            .collect::<SegmentTree<Max>>();
        println!("{:?}", st);
        assert_eq!(st.get(0..6).unwrap(), 365);
        assert_eq!(st.get(3..4).unwrap(), 365);
        assert_eq!(st.get(0..3).unwrap(), 6);
        assert_eq!(st.get(3..5).unwrap(), 365);
    }

    #[test]
    fn builder_1() {
        // [0, 0, 10, 24, 0, 12]
        let set_query = vec![0, 0, 10, 24, 0, 12];
        let mut segtree = set_query.into_iter().collect::<SegmentTree<Gcd>>();
        assert_eq!(segtree.get(0..6), 2);
        assert_eq!(segtree.get(2..4), 2);
        assert_eq!(segtree.get(3..6), 12);
        // [0, 25, 10, 24, 0, 12]
        segtree.asssign(1, 25);
        assert_eq!(segtree.get(1..3), 5);
        assert_eq!(segtree.get(0..6), 1);
    }
}
