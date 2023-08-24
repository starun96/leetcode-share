mod container_most_water;
mod search_2d_matrix;

fn main() {
    let matrix = vec![vec![2, 3, 4], vec![2, 4, 3], vec![4, 2, 3], vec![3, 2, 4]];
    let target = 8;
    search_2d_matrix::search_2d_matrix(matrix, target);
    println!("The result is: {result}");
}
