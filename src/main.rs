use crate::container_most_water::container_most_water_loop;

mod container_most_water;

fn main() {
    let heights = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    //let result = container_most_water::container_most_water();
    //println!("The result is: {result}");

    let result = container_most_water_loop(heights);
    println!("The result is: {result}");
}
