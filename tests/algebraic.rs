#[cfg(test)]
mod algebraic_test {
    use snippet::algebraic::*;
    #[test]
    fn add() {
        assert_eq!(Add::identity(), 0);
        assert_eq!(Add::operate(&2, &3), 5);
        assert_eq!(Add::inverse(&234), -234);
    }
    #[test]
    fn gcd() {
        assert_eq!(Gcd::identity(), 0);
        assert_eq!(Gcd::operate(&117, &57), 3);
        assert_eq!(Gcd::operate(&4541154, &Gcd::identity()), 4541154);
    }

    #[test]
    fn max() {
        assert_eq!(
            MaxElm::NegInf.partial_cmp(&MaxElm::Finite(0)),
            Some(std::cmp::Ordering::Less)
        );
        assert_eq!(
            MaxElm::Finite(0).partial_cmp(&Max::identity()),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(
            Max::operate(&MaxElm::Finite(111111), &MaxElm::Finite(44)),
            MaxElm::Finite(111111)
        );
    }
    #[test]
    fn max2() {
        assert_eq!(
            Max::operate(&MaxElm::NegInf, &MaxElm::Finite(-144154)),
            MaxElm::Finite(-144154)
        );
        assert_eq!(
            Max::operate(&(-144154).into(), &Max::identity()),
            (-144154).into()
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            Min::operate(&MinElm::Finite(111111), &MinElm::Finite(44)),
            MinElm::Finite(44)
        );
        assert_eq!(
            Min::operate(&MinElm::Finite(-144154), &Min::identity()),
            MinElm::Finite(-144154)
        );
    }
}
