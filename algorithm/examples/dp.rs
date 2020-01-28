use algorithm;

fn main() {
    greedy();
    memo();
}

fn greedy() {
    let input = vec![(2, 3), (1, 2), (3, 4), (2, 2)];
    let ans = algorithm::dp::greedy(&input, 0, 5);
    assert_eq!(7, ans);
}

fn memo() {
    let input = vec![(2, 3), (1, 2), (3, 4), (2, 2)];
    let mut table: Vec<Vec<i32>> = vec![vec![-1; input.len()]; input.len()];
    let ans = algorithm::dp::memo(&mut table, &input, 0, 5);
    assert_eq!(7, ans);
}
