use std::cell::OnceCell;
use std::ranges::Range;

fn do_something() {
    println!("Something!");
}

fn container_most_water(heights: Vec<usize>) -> usize {
    let n = heights.len();
    let mut solutions: Vec<Vec<OnceCell<usize>>> = vec![vec![OnceCell::new(); n]; n];

    return container_water_recursive(&heights, &solutions, 0, n - 1);
    // solutions[i, j] = max(solutions[i - 1, j] , solutions[i, j - 1])

    // if solution[i, j] already exists, then return solution[i, j]
    // else, compute solution[i, j] -> abs(i - j) * min(heights[i], heights[j])
}

/// This is a structure that wraps the (i, j)
struct Container<'a> {
    extents: &'a [usize],
}

///https://www.reddit.com/r/Fedora/comments/uy1wxr/rust_libssl_issue/

impl<'a> Container<'a> {
    pub fn area(&self) -> usize {
        if self.extents.is_empty() {
            return 0;
        }

        let left = self.extents.first();
        let right = self.extents.last();

        let width = self.extents.len();
        let height = std::cmp::min(left, right);
        return width * height;
    }
}

fn container_water_recursive(
    heights: &[usize],
    solutions: &mut Vec<Vec<OnceCell<usize>>>,`dasdasdadasddd6
    window: Range<usize>, // half open interval
) -> usize {
    // separation of pointers by 1 or more
    let len = window.end - window.start;
    if !window.is_empty() {

        // no window == no area
    } else {
        0
    }

    if j == window.start + 1 {
        let area = std::cmp::min(heights[i], heights[j]);
        solutions[i][j] = area;
        return area;
    }

    todo!()
}
