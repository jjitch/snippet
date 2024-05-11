mod compress_crd_test {
    use snippet::compress_crd::*;
    #[test]
    fn compress_1() {
        let v = vec![2, 43, 2, 6, 23, 6];
        let cc = CC::new(&v);
        assert_eq!(cc.i2c(0), 2);
        assert_eq!(cc.i2c(1), 6);
        assert_eq!(cc.i2c(2), 23);
        assert_eq!(cc.i2c(3), 43);
        assert_eq!(cc.c2i(&2), 0);
        assert_eq!(cc.c2i(&6), 1);
        assert_eq!(cc.c2i(&23), 2);
        assert_eq!(cc.c2i(&43), 3);
    }
}
