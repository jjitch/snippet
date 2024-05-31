/// # Extended GCD
/// * Solve linear indeterminate equation.
///   * ax + by = d; d = gcd(a, b)
/// * Given a and b, then returns (d, x, y).
pub fn xgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, x, y) = xgcd(b, a % b);
        (d, y, -a / b * y + x)
    }
}
