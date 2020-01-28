pub fn greedy(input: &[(u32, u32)], index: usize, weight: u32) -> u32 {
    match input.get(index) {
        None => 0,
        Some((w, _)) if weight < *w => greedy(input, index + 1, weight),
        Some((w, v)) => std::cmp::max(
            greedy(input, index + 1, weight),
            greedy(input, index + 1, weight - w) + v,
        ),
    }
}

pub fn memo(table: &mut Vec<Vec<i32>>, input: &[(i32, i32)], index: usize, weight: i32) -> i32 {
    if table[index][weight as usize] >= 0 {
        return table[index][weight as usize];
    };
    table[index][weight as usize] = match input.get(index) {
        None => 0,
        Some((w, _)) if weight < *w => memo(table, input, index + 1, weight),
        Some((w, v)) => std::cmp::max(
            memo(table, input, index + 1, weight),
            memo(table, input, index + 1, weight - w) + v,
        ),
    };
    table[index][weight as usize]
}
