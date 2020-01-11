fn _partial_sum(sum: i32, v: &[i32], k: i32, i: usize) -> bool {
    let next = match v.get(i) {
        Some(next) => next,
        None => return sum == k,
    };

    let j = i + 1;
    _partial_sum(sum + next, v, k, j) || _partial_sum(sum, v, k, j)
}

pub fn partial_sum(v: &[i32], k: i32) -> bool {
    if v.is_empty() {
        return false;
    }
    _partial_sum(0, v, k, 0)
}
