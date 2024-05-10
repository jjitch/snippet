mod compress_crd_test {
    use std::collections::BTreeMap;

    use snippet::compress_crd::*;
    #[test]
    fn compress_1() {
        let v = vec![1 << 60, 1 << 40, 1 << 60, 0, 1 << 40, 1 << 60, 1 << 50];
        let (uncomp, compress) = compress_crd(&v);
        assert_eq!(uncomp, vec![0, 1 << 40, 1 << 50, 1 << 60]);
        assert_eq!(
            compress,
            vec![(0, 0), (1 << 40, 1), (1 << 50, 2), (1 << 60, 3)]
                .into_iter()
                .collect::<BTreeMap<i64, usize>>()
        );
    }
}
