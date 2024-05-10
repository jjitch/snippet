pub trait Magma<T> {
    fn operate(x: &T, y: &T) -> T;
}

pub trait Identity<T> {
    fn identity() -> T;
}

pub trait Inverse<T> {
    fn inverse(x: &T) -> T;
}

pub trait Monoid<T>: Magma<T> + Identity<T> {}
impl<T, M> Monoid<T> for M where M: Magma<T> + Identity<T> {}

pub trait Group<T>: Monoid<T> + Inverse<T> {}
impl<T, G> Group<T> for G where G: Monoid<T> + Inverse<T> {}

pub mod mount {

    use num::Integer;

    use super::*;

    #[derive(Debug)]
    pub struct Add;

    impl Magma<i64> for Add {
        fn operate(x: &i64, y: &i64) -> i64 {
            x + y
        }
    }

    impl Identity<i64> for Add {
        fn identity() -> i64 {
            0
        }
    }

    impl Inverse<i64> for Add {
        fn inverse(x: &i64) -> i64 {
            -x
        }
    }

    #[derive(Debug)]
    pub struct Gcd;
    impl Magma<i64> for Gcd {
        fn operate(x: &i64, y: &i64) -> i64 {
            x.gcd(y)
        }
    }

    impl Identity<i64> for Gcd {
        fn identity() -> i64 {
            0
        }
    }

    #[derive(Debug)]
    pub struct Max;
    impl Magma<i64> for Max {
        fn operate(x: &i64, y: &i64) -> i64 {
            *x.max(y)
        }
    }

    impl Identity<i64> for Max {
        fn identity() -> i64 {
            -(1i64 << 60)
        }
    }

    #[derive(Debug)]
    pub struct Min;
    impl Magma<i64> for Min {
        fn operate(x: &i64, y: &i64) -> i64 {
            *x.min(y)
        }
    }

    impl Identity<i64> for Min {
        fn identity() -> i64 {
            1i64 << 60
        }
    }
}
