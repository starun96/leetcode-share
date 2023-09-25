use std::collections::VecDeque;

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut num_islands = 0;

    
    let m = grid.len();
    let n = grid[0].len();
    let mut furthest_visited = 0;
    while furthest_visited < m * n - 1 {
        let visited = VecDeque::new();

        while 
    }

    todo!()
}
