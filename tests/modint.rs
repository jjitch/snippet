mod modint_test {
    use competitive::modint::*;
    #[test]
    fn add_1() {
        let m = Mint::new(4);
        let m2 = mint(1);
        assert_eq!(m, m2 + 3);
    }

    #[test]
    fn test_add1() {
        assert_eq!(Mint::new(10) + 2, Mint::new(12));
    }

    #[test]
    fn test_sub1() {
        let a = Mint::new(8);
        let b = Mint::new(9);
        assert_eq!(a - b, GenericMint::new(Mod1000000007::modulus() - 1));
    }

    #[test]
    fn test_pow1() {
        assert_eq!(mint(2).pow(10), mint(1024));
    }

    #[test]
    fn test_pow2() {
        assert_eq!(mint(2).pow(9), mint(512));
    }

    #[test]
    fn test_mod_inv_large_m1() {
        assert_eq!(
            Mint::new(12345678900000) / Mint::new(100_000),
            Mint::new(123456789)
        );
    }

    #[test]
    fn test_mod_inv_large_m2() {
        assert_eq!(mint(12345678900000) / mint(100_000), mint(123456789));
    }

    #[test]
    fn test_macro1() {
        assert_eq!(mint(1 + 8), Mint::new(9));
    }

    #[test]
    fn test_inv_pow1() {
        assert_eq!(mint(89).inv(), mint(89).pow(-1));
    }

    #[test]
    fn test_inv_pow2() {
        assert_eq!(mint(89).inv().pow(2), mint(89).pow(-2));
    }

    #[test]
    fn add_assign1() {
        let mut i = mint(1);
        i += i;
        assert_eq!(i, mint(2));
    }
}
