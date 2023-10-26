use std::cmp::Ordering;

pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let num_cars = position.len();
    let mut pos_times_ordered = position
        .into_iter()
        .zip(speed.into_iter())
        .map(|(pos, spd)| (pos, ((target - pos) as f32) / (spd as f32)))
        .collect::<Vec<_>>();

    pos_times_ordered.sort_by(|(pos1, _), (pos2, _)| pos1.cmp(pos2));

    let mut num_fleets = 1;

    for i in (0..num_cars - 1).rev() {
        let (_, btime) = pos_times_ordered[i];
        let (_, ftime) = pos_times_ordered[i + 1];

        if btime <= ftime {
            pos_times_ordered[i].1 = ftime;
        } else {
            num_fleets += 1;
        }
    }

    num_fleets
}

pub fn car_fleet2(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars: Vec<_> = position.into_iter().zip(speed.into_iter()).collect();

    cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut num_fleets = 0;
    let mut front_time_to_arrival = 0f32;

    for (car_pos, car_spd) in cars {
        let time_to_arrival = (target - car_pos) as f32 / car_spd as f32;

        if time_to_arrival > front_time_to_arrival {
            front_time_to_arrival = time_to_arrival;
            num_fleets += 1;
        }
    }

    num_fleets
}
