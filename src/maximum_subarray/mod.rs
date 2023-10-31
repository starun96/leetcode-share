pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut total_max = nums[0];
    let mut running_max = 0;

    for elem in nums.iter() {
        if running_max <= 0 {
            running_max = *elem;
        } else {
            running_max += elem;
        }

        if running_max > total_max {
            total_max = running_max;
        }
    }

    return total_max;
}

// --------------------------------

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    return max_sub_array_sum(&nums, 0, nums.len() - 1);
}

fn max_sum_crossing(nums: &Vec<i32>, mid: usize, lo: usize, hi: usize) -> i32 {
    let mut bestLeft: i32 = i32::MIN;
    let mut v: i32 = 0;
    for i in (lo..mid + 1).rev() {
        v = v + nums[i];
        bestLeft = max(v, bestLeft);
    }
    let mut bestRight: i32 = i32::MIN;
    v = 0;
    for i in (mid + 1..hi + 1) {
        v = v + nums[i];
        bestRight = max(v, bestRight);
    }
    return bestLeft + bestRight;
}

fn max_sub_array_sum(nums: &Vec<i32>, lo: usize, hi: usize) -> i32 {
    if (hi == lo) {
        return nums[lo];
    }
    let mid = (lo + hi) / 2;
    let leftSum = max_sub_array_sum(nums, lo, mid);
    let rightSum = max_sub_array_sum(nums, mid + 1, hi);
    let crossingSum = max_sum_crossing(nums, mid, lo, hi);
    return max(max(leftSum, rightSum), crossingSum);
}

// ----------------------
use std::cmp::max;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let (mut maxsum, mut cursum) = (i32::MIN, 0);

    for num in nums {
        cursum = max(cursum + num, num);
        maxsum = max(maxsum, cursum);
    }

    return maxsum;
}
