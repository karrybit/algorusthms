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
