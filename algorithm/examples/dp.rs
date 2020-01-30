use algorithm;

fn main() {
    greedy();
    memo();
    dp();
}

fn greedy() {
    println!("--- greedy ---");
    let input = vec![(2, 3), (1, 2), (3, 4), (2, 2)];
    let ans = algorithm::dp::greedy(&input, 0, 5);
    println!("{}", ans);
}

fn memo() {
    println!("--- memo ---");
    let input = vec![(2, 3), (1, 2), (3, 4), (2, 2)];
    let weight = 5;
    let mut table: Vec<Vec<Option<i32>>> = vec![vec![None; weight + 1]; input.len() + 1];
    let ans = algorithm::dp::memo(&mut table, &input, 0, weight);
    println!("{}", ans.unwrap());
}

fn dp() {
    println!("--- dp ---");
    let input = vec![(2, 3), (1, 2), (3, 4), (2, 2)];
    let weight = 5;
    let mut table: Vec<Vec<i32>> = vec![vec![0; weight + 1]; input.len() + 1];
    let ans = algorithm::dp::dp(&mut table, &input, weight);
    println!("{}", ans);
}
