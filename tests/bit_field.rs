mod bit_field_test {
    use snippet::bit_field::*;
    #[test]
    fn none_test() {
        let s = 0b0;
        assert!(s.none());
    }
}
