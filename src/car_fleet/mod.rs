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
    // Sort the position, speed pairs.
    let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();

    // We use unstable sort because faster and we are guaranteed all starting positions are unique.
    cars.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    // We can use a vec as a stack.
    let mut fleets = Vec::new();

    // Iterate in reverse so we start from the car leading the pack.
    for (car_position, car_speed) in cars.iter().rev() {
        let time_to_arrival = (target - car_position) as f64 / *car_speed as f64;

        // Get the speed of the car in front of this car.
        if let Some(frontrunner_arrival_time) = fleets.last() {
            if (time_to_arrival > *frontrunner_arrival_time) {
                // This car will be its own fleet because its slower than the car in front of it
                fleets.push(time_to_arrival);
            } else {
                // This car will be absorbed into the fleet in front of it.
            }
        } else {
            // No car in front of this car
            fleets.push(time_to_arrival);
        }
    }

    fleets.len() as i32
}
