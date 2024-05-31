#[cfg(test)]
mod xgcd_test {
    use snippet::xgcd::xgcd;

    #[test]
    fn t1() {
        assert_eq!(xgcd(111, 30), (3, 3, -11));
    }
    #[test]
    fn t2() {
        let (a, b) = (4325, 653);
        let (d, x, y) = xgcd(a, b);
        assert_eq!(a * x + b * y, d);
    }
    #[test]
    fn t3() {
        let (a, b) = (4325, -653);
        let (d, x, y) = xgcd(a, b);
        assert_eq!(a * x + b * y, d);
    }
    #[test]
    fn t4() {
        let (a, b) = (-4325, -653);
        let (d, x, y) = xgcd(a, b);
        assert_eq!(a * x + b * y, d);
    }
}
