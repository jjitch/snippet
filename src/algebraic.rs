pub trait Monoid {
    type Element;
    fn operate(x: &Self::Element, y: &Self::Element) -> Self::Element;
    fn identity() -> Self::Element;
}

pub trait Group: Monoid {
    fn inverse(x: &Self::Element) -> Self::Element;
}

pub trait Act: Monoid {
    type Target;
    fn act(x: &Self::Target, m: &Self::Element) -> Self::Target;
    fn proportional(m: &Self::Element, n: usize) -> Self::Element;
}

pub struct Add;

impl Monoid for Add {
    type Element = i64;
    fn identity() -> Self::Element {
        0
    }
    fn operate(x: &Self::Element, y: &Self::Element) -> Self::Element {
        x + y
    }
}

impl Group for Add {
    fn inverse(x: &Self::Element) -> Self::Element {
        -x
    }
}

impl Act for Add {
    type Target = i64;
    fn act(m: &Self::Target, x: &Self::Element) -> Self::Target {
        m + x
    }
    fn proportional(m: &Self::Element, n: usize) -> Self::Element {
        m * n as i64
    }
}

pub struct Min;
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MinElm {
    Finite(i64),
    Infinite,
}

impl std::fmt::Debug for MinElm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &MinElm::Finite(x) => x.to_string(),
                &MinElm::Infinite => "INF".to_string(),
            }
        )
    }
}

impl From<i64> for MinElm {
    fn from(value: i64) -> Self {
        Self::Finite(value)
    }
}

impl std::ops::Add for MinElm {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MinElm::Finite(x), MinElm::Finite(y)) => MinElm::Finite(x + y),
            _ => MinElm::Infinite,
        }
    }
}

impl MinElm {
    pub fn unwrap(&self) -> i64 {
        match self {
            &MinElm::Finite(i) => i,
            &MinElm::Infinite => panic!("The content is INFINITE."),
        }
    }
}

impl Monoid for Min {
    type Element = MinElm;
    fn identity() -> Self::Element {
        MinElm::Infinite
    }
    fn operate(x: &Self::Element, y: &Self::Element) -> Self::Element {
        *x.min(y)
    }
}

impl Act for Min {
    type Target = i64;
    fn act(x: &Self::Target, m: &Self::Element) -> Self::Target {
        match *m {
            MinElm::Finite(m) => (*x).min(m),
            MinElm::Infinite => *x,
        }
    }
    fn proportional(m: &Self::Element, _: usize) -> Self::Element {
        *m
    }
}

pub struct Max;
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MaxElm {
    NegInf,
    Finite(i64),
}

impl std::fmt::Debug for MaxElm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &MaxElm::Finite(x) => x.to_string(),
                &MaxElm::NegInf => "-INF".to_string(),
            }
        )
    }
}

impl From<i64> for MaxElm {
    fn from(value: i64) -> Self {
        Self::Finite(value)
    }
}

impl std::ops::Add for MaxElm {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (MaxElm::Finite(x), MaxElm::Finite(y)) => MaxElm::Finite(x + y),
            _ => MaxElm::NegInf,
        }
    }
}

impl MaxElm {
    pub fn unwrap(&self) -> i64 {
        match self {
            &MaxElm::Finite(i) => i,
            &MaxElm::NegInf => panic!("The content is INFINITE."),
        }
    }
}

impl Monoid for Max {
    type Element = MaxElm;
    fn identity() -> Self::Element {
        MaxElm::NegInf
    }
    fn operate(x: &Self::Element, y: &Self::Element) -> Self::Element {
        *x.max(y)
    }
}

impl Act for Max {
    type Target = i64;
    fn act(x: &Self::Target, m: &Self::Element) -> Self::Target {
        match *m {
            MaxElm::Finite(m) => (*x).max(m),
            MaxElm::NegInf => *x,
        }
    }
    fn proportional(m: &Self::Element, _: usize) -> Self::Element {
        *m
    }
}

pub struct Gcd;

impl Monoid for Gcd {
    type Element = i64;
    fn identity() -> Self::Element {
        0
    }
    fn operate(x: &Self::Element, y: &Self::Element) -> Self::Element {
        if *y == 0 {
            *x
        } else {
            Self::operate(y, &(x % y))
        }
    }
}

pub struct Assign<T>(std::marker::PhantomData<fn() -> T>);

impl<T: Clone> Monoid for Assign<T> {
    type Element = Option<T>;
    fn identity() -> Self::Element {
        None
    }
    fn operate(x: &Self::Element, y: &Self::Element) -> Self::Element {
        if y.is_some() {
            y.clone()
        } else {
            x.clone()
        }
    }
}

impl<T: Clone> Act for Assign<T> {
    type Target = T;
    fn act(x: &Self::Target, m: &Self::Element) -> Self::Target {
        match m {
            &Some(ref t) => t.clone(),
            &None => x.clone(),
        }
    }
    fn proportional(m: &Self::Element, _: usize) -> Self::Element {
        m.clone()
    }
}

pub struct GeneralAdd<T>(std::marker::PhantomData<fn() -> T>);

impl Monoid for GeneralAdd<MinElm> {
    type Element = MinElm;
    fn identity() -> Self::Element {
        MinElm::Finite(0)
    }
    fn operate(x: &Self::Element, y: &Self::Element) -> Self::Element {
        *x + *y
    }
}

impl Act for GeneralAdd<MinElm> {
    type Target = MinElm;
    fn act(x: &Self::Target, m: &Self::Element) -> Self::Target {
        *x + *m
    }
    fn proportional(m: &Self::Element, n: usize) -> Self::Element {
        match m {
            MinElm::Finite(x) => MinElm::Finite(*x * n as i64),
            _ => MinElm::Infinite,
        }
    }
}

impl Monoid for GeneralAdd<MaxElm> {
    type Element = MaxElm;
    fn identity() -> Self::Element {
        MaxElm::Finite(0)
    }
    fn operate(x: &Self::Element, y: &Self::Element) -> Self::Element {
        *x + *y
    }
}

impl Act for GeneralAdd<MaxElm> {
    type Target = MaxElm;
    fn act(x: &Self::Target, m: &Self::Element) -> Self::Target {
        *x + *m
    }
    fn proportional(m: &Self::Element, n: usize) -> Self::Element {
        match m {
            MaxElm::Finite(x) => MaxElm::Finite(*x * n as i64),
            _ => MaxElm::NegInf,
        }
    }
}
