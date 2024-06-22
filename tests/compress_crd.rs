mod compress_crd_test {
    use snippet::compress_crd::*;
    #[test]
    fn compress_1() {
        let v = vec![2, 43, 2, 6, 23, 6];
        let cc = CC::from_iter(v.into_iter());
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
        let cc = CC::from_iter('a'..='z');
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

    #[test]
    fn vec_ref() {
        let k = 3;
        let cc = (0..1 << k)
            .map(|s| {
                (0..k)
                    .map(|i| if s & 1 << i == 0 { 'A' } else { 'B' })
                    .collect::<Vec<char>>()
            })
            .collect::<CC<_>>();
        assert_eq!(0, cc.c2i(&vec!['A', 'A', 'A'][..]));
        assert_eq!(0, cc.c2i(&vec!['A', 'A', 'A', 'A'][..3]));
    }
}
