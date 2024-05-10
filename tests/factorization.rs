mod factorization_test {
    use snippet::factorization::*;
    use std::collections::BTreeMap;
    fn vec2map(v: Vec<(i64, i64)>) -> BTreeMap<i64, i64> {
        v.into_iter().collect()
    }
    #[test]
    fn simple_1() {
        assert_eq!(factorization(10), vec2map(vec![(2, 1), (5, 1)]))
    }

    #[test]
    fn prime_1() {
        assert_eq!(factorization(7), vec2map(vec![(7, 1)]));
    }

    #[test]
    fn square_1() {
        assert_eq!(factorization(25), vec2map(vec![(5, 2)]));
    }

    #[test]
    fn two_powers() {
        assert_eq!(factorization(2048), vec2map(vec![(2, 11)]));
    }
}
