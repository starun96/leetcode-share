pub fn find_min(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }

    let mut start = 0;
    let mut end = n - 1;

    loop {
        let middle_idx = (start + end) / 2;
        let middle_elem = &nums[middle_idx];

        let end_elem = &nums[end];

        let prev_idx = middle_idx.checked_sub(1).unwrap_or(n - 1);
        let prev_elem = &nums[prev_idx];

        if middle_elem < prev_elem {
            return *middle_elem;
        }
        if middle_elem > end_elem {
            start = middle_idx + 1;
        } else {
            end = middle_idx.checked_sub(1).unwrap_or(0);
        }
    }
}
