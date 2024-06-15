// change modulus
pub type Mint = GenericMint<Mod1000000007>;
// pub type Mint = GenericMint<Mod998244353>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenericMint<M: Modulus>(i64, std::marker::PhantomData<fn() -> M>);

impl<M: Modulus + Copy> GenericMint<M> {
    pub fn new(x: i64) -> GenericMint<M> {
        GenericMint((x + M::modulus()) % M::modulus(), std::marker::PhantomData)
    }
    pub fn raw(x: i64) -> GenericMint<M> {
        GenericMint(x, std::marker::PhantomData)
    }
    pub fn pow(self, p: i64) -> GenericMint<M> {
        if p == 0 {
            GenericMint::new(1)
        } else {
            let s = if p < 0 { self.inv() } else { self };
            let p = if p < 0 { -p } else { p };
            let e = Self::pow(s, p >> 1);
            if p & 1 == 1 {
                e * e * s
            } else {
                e * e
            }
        }
    }
    pub fn inv(self) -> GenericMint<M> {
        let mut a = self.0;
        let (mut b, mut u, mut v) = (M::modulus(), 1, 0);
        while b > 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        Self::new(u)
    }
}

impl<T, M> std::ops::Add<T> for GenericMint<M>
where
    T: Into<GenericMint<M>>,
    M: Modulus + Copy,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self::new(self.0 + rhs.into().0)
    }
}

impl<T, M> std::ops::Sub<T> for GenericMint<M>
where
    T: Into<GenericMint<M>>,
    M: Modulus + Copy,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self::new(self.0 - rhs.into().0)
    }
}

impl<T, M> std::ops::Mul<T> for GenericMint<M>
where
    T: Into<GenericMint<M>>,
    M: Modulus + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.0 * rhs.into().0)
    }
}

impl<T, M> std::ops::Div<T> for GenericMint<M>
where
    T: Into<GenericMint<M>>,
    M: Modulus + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self * rhs.into().inv()
    }
}

impl<T, M> std::ops::AddAssign<T> for GenericMint<M>
where
    T: Into<GenericMint<M>>,
    M: Modulus + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T, M> std::ops::SubAssign<T> for GenericMint<M>
where
    T: Into<GenericMint<M>>,
    M: Modulus + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T, M> std::ops::MulAssign<T> for GenericMint<M>
where
    T: Into<GenericMint<M>>,
    M: Modulus + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T, M> std::ops::DivAssign<T> for GenericMint<M>
where
    T: Into<GenericMint<M>>,
    M: Modulus + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T, M: Modulus + Clone> std::ops::Index<GenericMint<M>> for Vec<T> {
    type Output = T;
    fn index(&self, index: GenericMint<M>) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl<T, M: Modulus + Copy> std::ops::IndexMut<GenericMint<M>> for Vec<T> {
    fn index_mut(&mut self, index: GenericMint<M>) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}

impl<M: Modulus + Copy> From<i64> for GenericMint<M> {
    fn from(x: i64) -> Self {
        Self::new(x)
    }
}

impl std::str::FromStr for Mint {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mint::new(i64::from_str(s)?))
    }
}

impl<M: Modulus + Copy> std::fmt::Display for GenericMint<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<M: Modulus + Copy> std::fmt::Debug for GenericMint<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait Modulus {
    fn modulus() -> i64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mod1000000007;

impl Modulus for Mod1000000007 {
    fn modulus() -> i64 {
        1_000_000_007_i64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mod998244353;

impl Modulus for Mod998244353 {
    fn modulus() -> i64 {
        998_244_353_i64
    }
}

pub fn mint(x: i64) -> Mint {
    Mint::new(x)
}

use super::algebraic::*;

pub struct ModAdd;

impl Monoid for ModAdd {
    type Element = Mint;
    fn identity() -> Self::Element {
        mint(0)
    }
    fn operate(x: &Self::Element, y: &Self::Element) -> Self::Element {
        *x + *y
    }
}

impl Group for ModAdd {
    fn inverse(x: &Self::Element) -> Self::Element {
        mint(0) - (*x)
    }
}

impl Act for ModAdd {
    type Target = Mint;
    fn act(m: &Self::Target, x: &Self::Element) -> Self::Target {
        *m + *x
    }
    fn proportional(m: &Self::Element, n: usize) -> Self::Element {
        *m * mint(n as i64)
    }
}

pub struct ModMul;

impl Monoid for ModMul {
    type Element = Mint;
    fn identity() -> Self::Element {
        mint(1)
    }
    fn operate(x: &Self::Element, y: &Self::Element) -> Self::Element {
        *x * *y
    }
}

impl Act for ModMul {
    type Target = Mint;
    fn act(x: &Self::Target, m: &Self::Element) -> Self::Target {
        *x * *m
    }
    fn proportional(m: &Self::Element, n: usize) -> Self::Element {
        m.pow(n as i64)
    }
}

impl Group for ModMul {
    fn inverse(x: &Self::Element) -> Self::Element {
        x.inv()
    }
}
