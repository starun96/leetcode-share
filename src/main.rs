mod car_fleet;

fn main() {
    let position = vec![10, 8, 0, 5, 3];
    let speed = vec![2, 4, 1, 10, 3];
    let target = 12;

    let position2 = vec![10, 2, 5, 7, 4, 6, 11];
    let speed2 = vec![7, 5, 10, 5, 9, 4, 1];
    let target2 = 13;

    let num_fleets = car_fleet::car_fleet(target2, position2, speed2);
    dbg!(num_fleets);
}
