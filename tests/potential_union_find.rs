#[cfg(test)]
mod potential_union_find_test {
    use snippet::{algebraic::*, potential_union_find::*};
    #[test]
    fn aoj() {
        let mut puf = PotentialUnionFind::<_, mount::Add>::new(5);
        puf.merge(0, 2, &5);
        println!("{:?}", puf);
        puf.merge(1, 2, &3);
        println!("{:?}", puf);
        assert_eq!(puf.diff(0, 1), Diff::Ok(2));
        assert_eq!(puf.diff(1, 3), Diff::Ambiguous);
        puf.merge(1, 4, &8);
        assert_eq!(puf.diff(0, 4), Diff::Ok(10));
    }
}
