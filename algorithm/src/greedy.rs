pub fn greedy_pay(coins: Vec<u32>, mut price: u32) -> u32 {
    let table = [1, 5, 10, 50, 100, 500];
    let mut count = 0;
    (0..=5).rev().for_each(|i| {
        let t = std::cmp::min(price / table[i], coins[i]);
        price -= t * table[i];
        count += t;
    });
    count
}
