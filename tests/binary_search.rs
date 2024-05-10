mod binary_search_test {
    use competitive::binary_search::*;

    // tests for generic binary search
    #[test]
    fn include_bs_1() {
        let f = |x: i64| 5 <= x;
        assert_eq!(binary_search(0..10i64, f), 5);
    }
    #[test]
    fn exclude_bs_2() {
        let f = |x: i64| 5 < x;
        assert_eq!(binary_search(0..10i64, f), 6);
    }
    #[test]
    fn large_bs_1() {
        let f = |x: i64| 5 <= x;
        assert_eq!(binary_search(0..1i64 << 62, f), 5);
    }
    #[test]
    fn out_of_range_bs_1() {
        let f = |x: i64| -1 <= x;
        assert_eq!(binary_search(0..10i64, f), 0);
    }
    #[test]
    fn out_of_range_bs_2() {
        let f = |x: i64| 20 <= x;
        assert_eq!(binary_search(0..10i64, f), 10);
    }

    // tests for lower bound
    #[test]
    fn simple_lower_bound_1() {
        let v = vec![0, 1, 2, 3, 4];
        assert_eq!(lower_bound(&v, 2), 2);
    }
    #[test]
    fn simple_lower_bound_2() {
        let v = vec![0, 2, 4, 6, 8];
        assert_eq!(lower_bound(&v, 3), 2);
    }
    #[test]
    fn out_of_range_lower_bound_1() {
        let v = vec![0, 1, 2, 3, 4];
        assert_eq!(lower_bound(&v, 10), 5);
    }
    #[test]
    fn out_of_range_lower_bound_2() {
        let v = vec![1, 1, 2, 3, 4];
        assert_eq!(lower_bound(&v, 0), 0);
    }
    #[test]
    fn lowest_lower_bound_1() {
        let v = vec![1, 1, 2, 3, 4];
        assert_eq!(lower_bound(&v, 1), 0);
    }
    #[test]
    fn highest_lower_bound_1() {
        let v = vec![1, 1, 2, 3, 4];
        assert_eq!(lower_bound(&v, 4), 4);
    }

    // tests for upper bound
    #[test]
    fn simple_upper_bound_1() {
        let v = vec![0, 1, 2, 3, 4];
        assert_eq!(upper_bound(&v, 2), 3);
    }
    #[test]
    fn simple_upper_bound_2() {
        let v = vec![0, 2, 4, 6, 8];
        assert_eq!(upper_bound(&v, 3), 2);
    }
    #[test]
    fn out_of_range_upper_bound_1() {
        let v = vec![0, 1, 2, 3, 4];
        assert_eq!(upper_bound(&v, 10), 5);
    }
    #[test]
    fn out_of_range_upper_bound_2() {
        let v = vec![1, 1, 2, 3, 4];
        assert_eq!(upper_bound(&v, 0), 0);
    }
    #[test]
    fn lowest_upper_bound_1() {
        let v = vec![1, 1, 2, 3, 4];
        assert_eq!(upper_bound(&v, 1), 2);
    }
    #[test]
    fn highest_upper_bound_1() {
        let v = vec![1, 1, 2, 3, 4];
        assert_eq!(upper_bound(&v, 4), 5);
    }

    #[test]
    fn simple_lower_bound_by_1() {
        assert_eq!(
            lower_bound_by(&[0, 4, 4, 4, 4, 4, 6], 4, std::cmp::Ord::cmp),
            1
        );
    }
}
