use algorithm;

fn setup_partial_sum() -> Vec<(Vec<i32>, i32, bool)> {
    vec![
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 55, true),
        (vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0, true),
        (vec![1], 1, true),
        (vec![], 0, false),
    ]
}

#[test]
fn test_is_partial_sum() {
    let cases = setup_partial_sum();
    cases.into_iter().enumerate().for_each(|(i, case)| {
        println!("[{}]: {:?}, {}", i, &case.0, case.1);
        let ans = algorithm::dfs::is_partial_sum(&case.0, case.1);
        assert_eq!(ans, case.2);
    });
}
