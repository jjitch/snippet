mod arg_cmp_test {
    use snippet::arg_cmp::*;
    use std::cmp::Ordering::*;
    #[test]
    fn bool_cmp() {
        assert_eq!(true.cmp(&false), Greater);
    }

    #[test]
    fn simple_arg_cmp() {
        let arr = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        let n = arr.len();
        for i in 0..n - 1 {
            assert_eq!(
                arg_cmp(&arr[i], &arr[i + 1]),
                Less,
                "{:?} {:?}",
                arr[i],
                arr[i + 1]
            );
        }
    }

    #[test]
    fn among_polar_axis_1() {
        assert_eq!(arg_cmp(&(1, 0), &(1, -1)), Less);
    }

    #[test]
    #[should_panic(expected = "can't compare with pole: (0, 0).")]
    fn pole_is_the_least_1() {
        assert_eq!(arg_cmp(&(0, 0), &(1, 0)), Less);
    }

    #[test]
    fn equal_1() {
        assert_eq!(arg_cmp(&(1, 1), &(2, 2)), Equal);
    }
}
