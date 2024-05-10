mod union_find_test {
    use snippet::union_find::*;
    #[test]
    fn unite_1() {
        let mut dsu = UnionFind::new(10);
        assert_eq!(dsu.size(0), 1);
        assert!(dsu.unite(0, 4));
        assert_eq!(dsu.size(0), 2);
        assert!(!dsu.unite(0, 4));
        assert!(dsu.unite(0, 5));
        assert!(dsu.is_same(4, 5));
    }

    #[test]
    fn size_1() {
        let mut dsu = UnionFind::new(5);
        assert!(dsu.unite(0, 1));
        assert!(dsu.unite(0, 2));
        assert!(dsu.unite(0, 3));
        assert!(dsu.unite(0, 4));
        assert_eq!(dsu.size(0), 5);
        assert_eq!(dsu.size(1), 5);
        assert_eq!(dsu.size(2), 5);
        assert_eq!(dsu.size(3), 5);
        assert_eq!(dsu.size(4), 5);
    }
}
