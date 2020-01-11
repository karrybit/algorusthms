use algorithm;

fn main() {
    let arr = [1, 2, 4, 7];
    let k = 13;
    println!(
        "{:?} can make up {} is {}",
        &arr,
        k,
        algorithm::dfs::partial_sum(&arr, k)
    );
}
