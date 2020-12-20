pub fn greedy(input: &[(u32, u32)], index: usize, weight: u32) -> u32 {
    println!("index: {}, weight: {}", index, weight);
    match input.get(index) {
        None => 0,
        Some((w, _)) if weight < *w => greedy(input, index + 1, weight),
        Some((w, v)) => std::cmp::max(
            greedy(input, index + 1, weight),
            greedy(input, index + 1, weight - w) + v,
        ),
    }
}

pub fn memo(
    table: &mut Vec<Vec<Option<i32>>>,
    input: &[(i32, i32)],
    index: usize,
    weight: usize,
) -> Option<i32> {
    println!("table: {:?}", table);
    println!("index: {}, weight: {}", index, weight);
    if let Some(value) = table.get(index).and_then(|_table| _table.get(weight)) {
        if value.map_or(false, |value| value >= 0) {
            return *value;
        }
    }

    table[index][weight] = if index == input.len() {
        Some(0)
    } else {
        match input.get(index) {
            None => Some(0),
            Some((w, _)) if weight < *w as usize => memo(table, input, index + 1, weight),
            Some((w, v)) => std::cmp::max(
                memo(table, input, index + 1, weight),
                memo(table, input, index + 1, weight - *w as usize).map(|value| value + v),
            ),
        }
    };
    table[index][weight]
}

pub fn dp(table: &mut Vec<Vec<i32>>, input: &[(i32, i32)], weight: usize) -> i32 {
    println!("{:?}", table);
    (0..input.len()).for_each(|i| {
        (0..=weight).for_each(|j| {
            println!(
                "input[{}].value: {}, input[{}].weight: {}, j: {}",
                i, input[i].1, j, input[i].0, j
            );
            table[i + 1][j] = if j < input[i].0 as usize {
                table[i][j]
            } else {
                std::cmp::max(table[i][j], table[i][j - input[i].0 as usize] + input[i].1)
            };
            println!("{:?}", table);
        })
    });
    println!("{:?}", table);
    table[input.len()][weight]
}
