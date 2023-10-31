use std::collections::VecDeque;

fn breadth_first_search(grid: &mut Vec<Vec<i32>>, start_r: usize, start_c: usize) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_front((start_r, start_c));

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut num_cells = 0;
    grid[start_r][start_c] = 0;

    while let Some((cur_r, cur_c)) = queue.pop_back() {
        // up

        if cur_r >= 1 && grid[cur_r - 1][cur_c] == 1 {
            grid[cur_r - 1][cur_c] = 0;
            queue.push_front((cur_r - 1, cur_c));
        }
        // down
        if cur_r + 2 <= num_rows && grid[cur_r + 1][cur_c] == 1 {
            grid[cur_r + 1][cur_c] = 0;
            queue.push_front((cur_r + 1, cur_c));
        }
        // left
        if cur_c >= 1 && grid[cur_r][cur_c - 1] == 1 {
            grid[cur_r][cur_c - 1] = 0;
            queue.push_front((cur_r, cur_c - 1));
        }
        // right
        if cur_c + 2 <= num_cols && grid[cur_r][cur_c + 1] == 1 {
            grid[cur_r][cur_c + 1] = 0;
            queue.push_front((cur_r, cur_c + 1));
        }
        num_cells += 1;
    }

    num_cells
}

pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut max_area_so_far = 0;
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    for i in 0..num_rows {
        for j in 0..num_cols {
            if grid[i][j] != 1 {
                continue;
            }

            let island_area = breadth_first_search(&mut grid, i, j);
            if island_area > max_area_so_far {
                max_area_so_far = island_area;
            }
        }
    }

    return max_area_so_far as i32;
}
