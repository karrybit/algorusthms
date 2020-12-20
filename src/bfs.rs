#[derive(Debug)]
pub enum MazeError {
    InvalidArgument,
}

pub fn solve_maze(
    maze: Vec<Vec<&str>>,
    m: usize,
    n: usize,
    start: (i32, i32),
    goal: (i32, i32),
) -> Result<i32, MazeError> {
    if maze.is_empty() {
        return Err(MazeError::InvalidArgument);
    }

    let mut table = vec![vec![std::i32::MAX; m]; n];
    table[start.0 as usize][start.1 as usize] = 0;

    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();
        if point.0 == goal.0 && point.1 == goal.1 {
            break;
        }
        dx.iter().zip(dy.iter()).for_each(|(x, y)| {
            let next = (point.0 + x, point.1 + y);
            if (0..m).contains(&(next.0 as usize))
                && (0..n).contains(&(next.1 as usize))
                && maze[next.0 as usize][next.1 as usize] != "#"
                && table[next.0 as usize][next.1 as usize] == std::i32::MAX
            {
                queue.push_front(next);
                table[next.0 as usize][next.1 as usize] =
                    table[point.0 as usize][point.1 as usize] + 1;
            }
        });
    }

    Ok(table[goal.0 as usize][goal.1 as usize])
}
