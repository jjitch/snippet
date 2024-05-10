pub fn binary_search<F>(range: std::ops::Range<i64>, judge: F) -> i64
where
    F: Fn(i64) -> bool,
{
    let mut high = range.end;
    let mut low = range.start;
    if !judge(high - 1) {
        high
    } else if judge(low) {
        low
    } else {
        while high - low != 1 {
            let mid = low + (high - low) / 2;
            if judge(mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        high
    }
}

pub fn lower_bound<T>(arr: &[T], val: T) -> usize
where
    T: std::cmp::Ord,
{
    let n = arr.len();
    binary_search(0..n as i64, |i| &val <= &arr[i as usize]) as usize
}

/// # lower bound with a custom compare function
/// Return the smallest `i` which meets `val <= arr[i]`.
/// ## Requirement
/// * `arr` needs to be sorted by `cmp`
/// ## example
/// ```
/// use competitive::binary_search::*;
/// fn simple_lower_bound_by_1() {
///     assert_eq!(
///         lower_bound_by(&[0, 4, 4, 4, 4, 4, 6],
///             4, std::cmp::Ord::cmp),
///         1
///     );
/// }
/// ```
///
pub fn lower_bound_by<T, F>(arr: &[T], val: T, cmp: F) -> usize
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let n = arr.len();
    binary_search(0..n as i64, |i| match cmp(&val, &arr[i as usize]) {
        std::cmp::Ordering::Less => true,
        std::cmp::Ordering::Equal => true,
        std::cmp::Ordering::Greater => false,
    }) as usize
}

pub fn upper_bound<T>(arr: &[T], val: T) -> usize
where
    T: std::cmp::Ord,
{
    let n = arr.len();
    binary_search(0..n as i64, |i| &val < &arr[i as usize]) as usize
}
