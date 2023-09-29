mod minimum_rotated_array;
use minimum_rotated_array::find_min;

fn main() {
    let nums1 = vec![1, 2, 3, 4, 5, 6, 7];
    let nums2 = vec![2, 3, 4, 5, 6, 7, 1];
    let nums3 = vec![4, 5, 6, 7, 1, 2, 3];
    let nums4 = vec![1];
    let res = find_min(nums4);
    println!("{res}");
}
