mod iter_extent_test {
    use competitive::iter_extent::*;
    #[test]
    fn count_1() {
        assert_eq!(
            vec![1i64, 1, 2, 1, 3, 3, 4, 1].into_iter().counts(),
            vec![(1i64, 4usize), (2, 1), (3, 2), (4, 1)]
                .into_iter()
                .collect()
        );
    }
}
