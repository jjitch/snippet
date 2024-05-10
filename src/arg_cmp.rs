pub fn arg_cmp(p: &(i64, i64), q: &(i64, i64)) -> std::cmp::Ordering {
    if *p == (0, 0) || *q == (0, 0) {
        panic!("can't compare with pole: (0, 0).")
    }
    (p.1 < 0)
        .cmp(&(q.1 < 0))
        .then_with(|| (p.1 * q.0).cmp(&(p.0 * q.1)))
}
