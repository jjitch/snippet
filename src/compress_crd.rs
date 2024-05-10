pub fn compress_crd(v: &Vec<i64>) -> (Vec<i64>, std::collections::BTreeMap<i64, usize>) {
    let uncomp: Vec<_> = v
        .iter()
        .map(|&v| v)
        .collect::<std::collections::BTreeSet<i64>>()
        .into_iter()
        .collect();
    let compress = uncomp
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    (uncomp, compress)
}
