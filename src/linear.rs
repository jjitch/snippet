/// manages a linear function : f(x) = a*x + b.
pub struct Linear {
    a: i64,
    b: i64,
}

impl Linear {
    pub fn new_with(a: i64, b: i64) -> Self {
        Self { a, b }
    }
}
