mod koko_eats_bananas;

fn main() {
    let piles = vec![2, 3, 4, 5];
    let h = 8;
    let result = koko_eats_bananas::solution::min_eating_speed(piles, h);
    dbg!(result);
}
