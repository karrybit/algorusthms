use algorithm;

fn main() {
    let coins = vec![3, 2, 1, 3, 0, 2];
    let price = 620;
    println!("{}", algorithm::greedy::greedy_pay(coins, price));
}
