pub fn factorization(x: i64) -> std::collections::BTreeMap<i64, i64> {
    let mut x = x;
    let mut i = 2;
    let mut map = std::collections::BTreeMap::new();
    while i * i <= x {
        if x % i == 0 {
            *map.entry(i).or_default() += 1i64;
            x /= i;
        } else {
            i += 1;
        }
    }
    if x > 1 {
        *map.entry(x).or_default() += 1;
    }
    map
}
