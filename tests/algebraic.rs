#[cfg(test)]
mod algebraic_test {
    use snippet::algebraic::*;
    #[test]
    fn add() {
        assert_eq!(mount::Add::identity(), 0);
        assert_eq!(mount::Add::operate(&2, &3), 5);
        assert_eq!(mount::Add::inverse(&234), -234);
    }
    #[test]
    fn gcd() {
        assert_eq!(mount::Gcd::identity(), 0);
        assert_eq!(mount::Gcd::operate(&117, &57), 3);
        assert_eq!(
            mount::Gcd::operate(&4541154, &mount::Gcd::identity()),
            4541154
        );
    }

    #[test]
    fn max() {
        assert_eq!(mount::Max::operate(&111111, &44), 111111);
        assert_eq!(
            mount::Max::operate(&-144154, &mount::Max::identity()),
            -144154
        );
    }
    #[test]
    fn min() {
        assert_eq!(mount::Min::operate(&111111, &44), 44);
        assert_eq!(
            mount::Min::operate(&-144154, &mount::Min::identity()),
            -144154
        );
    }
}
