#[inline]
fn div_ceil(n: u32, d: u32) -> u32 {
    (n + d - 1) / d
}

#[inline]
fn can_finish(piles: &Vec<i32>, k: i32, h: i32) -> bool {
    let mut remaining_hours = h;
    for &pile in piles {
        let eats = div_ceil(pile as u32, k as u32);
        remaining_hours -= eats as i32;

        if remaining_hours < 0 {
            return false;
        }
    }
    true
}

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut start = 1;
    let mut end = *piles.iter().max().expect("Piles is empty");

    while start < end {
        let k = (start + end) / 2;

        if can_finish(&piles, k, h) {
            end = k;
        } else {
            start = k + 1;
        }
    }

    return start;
}
