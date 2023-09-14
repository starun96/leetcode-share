mod top_k_frequent_elements;

fn main() {
    let nums = vec![1, 4, 3, 4, 4, 5, 2, 0, 5, 8, 2];
    let k = 3;
    let result = top_k_frequent_elements::top_k_frequent(nums, k);

    dbg!(result);
}
