mod container_most_water;
mod search_2d_matrix;

fn main() {
    let matrix = vec![vec![1]];
    let target = 0;
    let result = search_2d_matrix::search_matrix(matrix, target);
    println!("The result is: {result}");
}
