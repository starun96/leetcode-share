use std::cmp::Ordering;

pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut mixed = position
        .into_iter()
        .zip(speed.into_iter())
        .collect::<Vec<_>>();

    mixed.sort_by(|(pos1, _), (pos2, _)| pos1.cmp(pos2));

    let num_cars = mixed.len();
    let mut num_fleets = 1;
    for i in (0..num_cars - 1).rev() {
        let (bpos, bspd) = mixed[i];
        let (fpos, fspd) = mixed[i + 1];
        let bval = fspd * (target - bpos);
        let fval = bspd * (target - fpos);
        match (bval).cmp(&fval) {
            Ordering::Greater => num_fleets += 1,
            Ordering::Less => mixed[i].1 = fspd,
            Ordering::Equal => (),
        }
    }

    num_fleets
}
