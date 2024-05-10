mod factors_test {
    use competitive::factors::*;
    #[test]
    fn twenty_four() {
        let mut facts = factors(24);
        assert_eq!(facts.next(), Some(1));
        assert_eq!(facts.next(), Some(2));
        assert_eq!(facts.next(), Some(3));
        assert_eq!(facts.next(), Some(4));
        assert_eq!(facts.next(), Some(6));
        assert_eq!(facts.next(), Some(8));
        assert_eq!(facts.next(), Some(12));
        assert_eq!(facts.next(), Some(24));
        assert_eq!(facts.next(), None);
    }

    #[test]
    fn square_num() {
        let mut facts = factors(25);
        assert_eq!(facts.next(), Some(1));
        assert_eq!(facts.next(), Some(5));
        assert_eq!(facts.next(), Some(25));
        assert_eq!(facts.next(), None);
    }

    #[test]
    fn one() {
        let mut facts = factors(1);
        assert_eq!(facts.next(), Some(1));
        assert_eq!(facts.next(), None);
    }
}
