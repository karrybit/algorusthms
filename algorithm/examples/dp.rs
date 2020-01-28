use algorithm;

fn main() {
    greedy();
}

fn greedy() {
    let input = vec![(2, 3), (1, 2), (3, 4), (2, 2)];
    assert_eq!(7, algorithm::dp::greedy(&input, 0, 5));
}
