use maximum_subarray::max_sub_array;

mod maximum_subarray;

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let result = max_sub_array(nums);
    dbg!(result);
}
