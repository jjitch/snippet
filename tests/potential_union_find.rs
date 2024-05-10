use snippet::potential_union_find::*;

#[test]
fn s() {
    let s = r#"5 6
0 0 2 5
0 1 2 3
1 0 1
1 1 3
0 1 4 8
1 0 4
    "#;
    let mut puf = PotentialUnionFind::new::<Add>(3);
    puf.merge::<Add>(0, 2, &5);
}

struct Add;

impl Monoid<i64> for Add {
    fn apply(x: &i64, y: &i64) -> i64 {
        *x + *y
    }
    fn identity() -> i64 {
        0
    }
}

impl Group<i64> for Add {
    fn inv(x: &i64) -> i64 {
        -(*x)
    }
}
