use std::cell::OnceCell;

fn do_something() {
    println!("Something!");
}

fn container_most_water(heights: Vec<usize>) -> usize {
    let N = heights.len();
    // let mut solutions: Vec<OnceCell<usize>> = vec![ [ OnceCell::new(); N]];

    return container_water_recursive(&heights, 0, N - 1);
    // solutions[i, j] = max(solutions[i - 1, j] , solutions[i, j - 1])

    // if solution[i, j] already exists, then return solution[i, j]
    // else, compute solution[i, j] -> abs(i - j) * min(heights[i], heights[j])



}

fn container_water_recursive(heights: &[usize], i: usize, j: usize) -> usize {
    if
}
