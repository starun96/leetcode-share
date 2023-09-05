#[inline]
fn div_ceil(n: u32, d: u32) -> u32 {
    n / d + if n % d > 0 { 1 } else { 0 }
}

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut start: u32 = 1; // piles.iter().map(|&p| div_ceil(p as u32, h as u32)).sum();
    let mut end = piles
        .iter()
        .map(|&p| p as u32)
        .max()
        .expect("Piles is empty");

    let mut k_lowest = u32::MAX;
    while start <= end {
        let k = (start + end) / 2;

        let t = piles
            .iter()
            .map(|&p| div_ceil(p as u32, k as u32))
            .sum::<u32>();

        if t > h as u32 {
            start = k + 1;
        } else {
            end = k - 1;

            k_lowest = std::cmp::min(k, k_lowest);
        }
    }

    return k_lowest as i32;
}
