mod compress_crd_test {
    use snippet::compress_crd::*;
    #[test]
    fn compress_1() {
        let v = vec![2, 43, 2, 6, 23, 6];
        let cc = CC::new(v.into_iter());
        assert_eq!(cc.i2c(0), 2);
        assert_eq!(cc.i2c(1), 6);
        assert_eq!(cc.i2c(2), 23);
        assert_eq!(cc.i2c(3), 43);
        assert_eq!(cc.c2i(&2), 0);
        assert_eq!(cc.c2i(&6), 1);
        assert_eq!(cc.c2i(&23), 2);
        assert_eq!(cc.c2i(&43), 3);
    }

    #[test]
    fn char_comp() {
        let cc = CC::new('a'..='z');
        assert_eq!(cc.c2i(&'a'), 0);
        assert_eq!(cc.c2i(&'b'), 1);
        assert_eq!(cc.c2i(&'c'), 2);
        assert_eq!(cc.c2i(&'d'), 3);
        assert_eq!(cc.c2i(&'z'), 25);
    }

    #[test]
    fn from_iter() {
        let cc = ('a'..='z').collect::<CC<char>>();
        assert_eq!(cc.c2i(&'a'), 0);
        assert_eq!(cc.c2i(&'b'), 1);
        assert_eq!(cc.c2i(&'c'), 2);
        assert_eq!(cc.c2i(&'d'), 3);
        assert_eq!(cc.c2i(&'z'), 25);
    }
}
