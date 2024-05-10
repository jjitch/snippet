/// Generates SCC (Strongly Connected Components) from the graph which is expressed as an adjacency list.
pub fn strongly_connected_component(g: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = g.len();
    let mut index_list: Vec<usize> = vec![0; n];
    let mut visited: Vec<bool> = vec![false; n];
    let mut id = 0;

    #[derive(Clone)]
    enum Dir {
        FORWARD(usize),
        BACKWARD(usize),
    }

    let mut stack = Vec::with_capacity(2 * g.len());

    for i in 0..n {
        if visited[i] {
            continue;
        }
        stack.push(Dir::BACKWARD(i));
        stack.push(Dir::FORWARD(i));
        while let Some(d) = stack.pop() {
            match d {
                Dir::FORWARD(i) => {
                    visited[i] = true;
                    for adj in &g[i] {
                        if visited[*adj] {
                            continue;
                        }
                        stack.push(Dir::BACKWARD(*adj));
                        stack.push(Dir::FORWARD(*adj));
                    }
                }
                Dir::BACKWARD(i) => {
                    index_list[id] = i;
                    id += 1;
                }
            }
        }
    }
    for i in 0..n {
        visited[i] = false;
    }
    let mut res = vec![];
    let mut backward = vec![vec![]; n];
    for i in 0..n {
        for e in &g[i] {
            backward[*e].push(i);
        }
    }

    let mut stack = vec![];

    for i in index_list.into_iter().rev() {
        if visited[i] {
            continue;
        }
        let mut component = vec![];
        stack.push(i);
        while let Some(v) = stack.pop() {
            visited[v] = true;
            component.push(v);
            for adj in &backward[v] {
                if visited[*adj] {
                    continue;
                }
                stack.push(*adj);
            }
        }
        res.push(component);
    }
    res
}

#[cfg(test)]
pub mod test {
    use super::strongly_connected_component;
    #[test]
    fn simple_graph() {
        let g = vec![
            vec![6],
            vec![5],
            vec![1],
            vec![1],
            vec![0],
            vec![7],
            vec![4],
            vec![2, 6],
        ];
        for (a, b) in strongly_connected_component(&g).iter().zip(vec![
            vec![3],
            vec![1, 5, 7, 2],
            vec![6, 4, 0],
        ]) {
            assert!(set_eq(a, &b));
        }
    }

    #[test]
    fn heavy_input() {
        let n = 2_000_000;
        let g = (0..n)
            .map(|i| if i == n - 1 { vec![0] } else { vec![i + 1] })
            .collect::<Vec<Vec<usize>>>();
        let s = strongly_connected_component(&g);
        // println!("{:?}", s);
        let ans = vec![(1..n).rev().fold(vec![0], |mut v, i| {
            v.push(i);
            v
        })];
        assert_eq!(s, ans);
    }

    #[test]
    fn debug_entry() {
        let g = vec![
            vec![6],
            vec![5],
            vec![1],
            vec![1],
            vec![0],
            vec![7],
            vec![4],
            vec![2, 6],
        ];
        let _ = strongly_connected_component(&g);
    }

    fn set_eq<T>(a: &Vec<T>, b: &Vec<T>) -> bool
    where
        T: PartialEq,
    {
        assert_eq!(a.len(), b.len());
        a.iter()
            .fold(true, |exist, v| exist && b.iter().any(|w| *w == *v))
    }

    #[test]
    fn test_set_eq_1() {
        assert!(set_eq(&vec![6, 7, 8], &vec![8, 7, 6]));
    }
    #[test]
    fn test_set_eq_2() {
        assert!(!set_eq(&vec![6, 7, 8], &vec![5, 7, 6]));
    }
}
