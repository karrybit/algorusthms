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

pub fn range_scheduling(mut processes: Vec<(u32, u32)>) -> u32 {
    processes.sort_by(|p1, p2| p1.1.cmp(&p2.1));
    let mut count = 0;
    let mut time = 0;
    for process in processes.iter() {
        if process.0 < time {
            continue;
        }
        count += 1;
        time = process.1;
    }
    count
}

pub fn best_cow_line(s: String) -> String {
    let mut t = "".to_string();
    let (mut front, mut back): (usize, usize) = (0, s.len() - 1);
    while front <= back {
        let l = s.chars().nth(front).unwrap();
        let r = s.chars().nth(back).unwrap();
        if l < r {
            t.push(l);
            front += 1;
        } else {
            t.push(r);
            back -= 1;
        }
    }
    t
}
