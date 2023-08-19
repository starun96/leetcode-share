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
