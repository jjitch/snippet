mod segment_tree_test {
    use snippet::segment_tree::*;
    #[test]
    fn add_1() {
        let mut segtree = SegmentTree::<Add>::new(10);
        segtree.set(0, 10);
        segtree.set(1, 10);
        segtree.set(3, 10);
        assert_eq!(segtree.get(1..2), 10);
        assert_eq!(segtree.get(1..3), 10);
        assert_eq!(segtree.get(1..4), 20);
        assert_eq!(segtree.get(0..4), 30);
    }
    #[test]
    fn min_1() {
        let set_query = vec![5, 6, 1, 365, 44, 6];
        let st = set_query
            .into_iter()
            .enumerate()
            .fold(SegmentTreeBuilder::<Min>::new(6), |st, (i, v)| st.set(i, v))
            .biuld();
        assert_eq!(st.get(0..6), 1);
        assert_eq!(st.get(3..4), 365);
        assert_eq!(st.get(0..3), 1);
        assert_eq!(st.get(3..5), 44);
    }
    #[test]
    fn max_1() {
        let set_query = vec![5, 6, 1, 365, 44, 6];
        let st = set_query
            .into_iter()
            .enumerate()
            .fold(SegmentTreeBuilder::<Max>::new(6), |st, (i, v)| st.set(i, v))
            .biuld();
        assert_eq!(st.get(0..6), 365);
        assert_eq!(st.get(3..4), 365);
        assert_eq!(st.get(0..3), 6);
        assert_eq!(st.get(3..5), 365);
    }

    #[test]
    fn builder_1() {
        // [0, 0, 10, 24, 0, 12]
        let set_query = vec![(3, 24), (2, 10), (5, 12), (5, 36)];
        let mut segtree = set_query
            .into_iter()
            .fold(SegmentTreeBuilder::<Gcd>::new(6), |builder, (i, val)| {
                builder.set(i, val)
            })
            .biuld();
        assert_eq!(segtree.get(0..6), 2);
        assert_eq!(segtree.get(2..4), 2);
        // [0, 25, 10, 24, 0, 12]
        segtree.set(1, 25);
        assert_eq!(segtree.get(1..3), 5);
        assert_eq!(segtree.get(0..6), 1);
    }
}
