use std::cell::OnceCell;
use std::ops::Range;

pub fn container_most_water(heights: Vec<usize>) -> usize {
    let n = heights.len();
    let solutions = vec![vec![OnceCell::new(); n]; n];

    return container_water_recursive(heights.as_slice(), &solutions, 0..n);
}

fn area(container: &[usize]) -> usize {
    let width = container.len();

    if width < 2 {
        return 0;
    }

    let left = container[0];
    let right = container[width - 1];

    let height = std::cmp::min(left, right);
    return width * height;
}

fn container_water_recursive(
    heights: &[usize],
    solutions: &Vec<Vec<OnceCell<usize>>>,
    window: Range<usize>, // half open interval
) -> usize {
    let Range { start, end } = window;
    let entry = &solutions[start][end - 1];
    let ans = entry.get_or_init(move || {
        if window.len() <= 2 {
            return area(&heights[window]);
        }

        let right_subset = (start + 1)..end;
        let right_subset_solution = container_water_recursive(heights, solutions, right_subset);

        let left_subset = start..(end - 1);
        let left_subset_solution = container_water_recursive(heights, solutions, left_subset);

        let best_solution = std::cmp::max(left_subset_solution, right_subset_solution);
        return best_solution;
    });
    *ans
}

pub fn container_most_water_loop(height: Vec<i32>) -> i32 {
    // number of lines
    let n = height.len();

    // if number of lines is less than 2, just return 0
    if n < 2 {
        return 0;
    }

    // set up solutions 2D vector for storing solutions for future computations
    let mut solutions = vec![vec![0; n]; n];

    // initialize solutions for every subslice in the array of width 1
    for i in 0..n - 1 {
        // area of this subslice will just be the height of the shorter line (Area = 1 x min(h1, h2))
        let area = std::cmp::min(&height[i], &height[i + 1]);
        solutions[i][i + 1] = *area;
    }

    // increase width of subslice window, and get max area for each subslice
    for width in 2..n {
        for j in 0..n - width {
            // get area for the "parent" container
            let container_area = width as i32 * std::cmp::min(&height[j], &height[j + width]);

            // get the left subset max solution
            let left_subset_area = solutions[j][j + width - 1];

            // get the right subset max solution
            let right_subset_area = solutions[j + 1][j + width];

            // get the max of the child solutions
            let child_max_area = std::cmp::max(left_subset_area, right_subset_area);

            // get the max out of the child and parent containers
            let total_max_area = std::cmp::max(container_area, child_max_area);

            // store the total max area as a solution
            solutions[j][j + width] = total_max_area;
        }
    }

    // return the max solution across the entire array
    return solutions[0][n - 1];
}
