use algorithm;

fn main() {
    let coins = vec![3, 2, 1, 3, 0, 2];
    let price = 620;
    println!("{}", algorithm::greedy::greedy_pay(coins, price));

    let processes = vec![(1, 3), (2, 5), (4, 7), (6, 9), (8, 10)];
    println!("{}", algorithm::greedy::range_scheduling(processes.clone()));

    println!("{}", algorithm::greedy::best_cow_line("ACDBCB".to_string()));

    let points = vec![1, 7, 15, 20, 30, 50];
    println!("{}", algorithm::greedy::sarumans_army(points, 10));

    let board = vec![8, 5, 8];
    println!("{}", algorithm::greedy::fence_repair(board));
}
