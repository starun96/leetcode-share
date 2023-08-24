fn get_element_from_idx(matrix: &Vec<Vec<i32>>, mid_idx: usize, num_rows: usize) -> i32 {
    let row = mid_idx / num_rows;
    let col = mid_idx % num_rows;

    return matrix[row][col];
}

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let num_rows = matrix.len();
    let num_cols = matrix.first().unwrap().len();
    let mut left = 0;
    let mut right = (num_rows * num_cols) - 1;

    while left <= right {
        let mid_idx = (left + right) / 2;
        let mid_element = get_element_from_idx(&matrix, mid_idx, num_rows);

        if mid_element == target {
            return true;
        } else if mid_element < target {
            left = mid_idx + 1;
        } else if mid_element > target {
            right = mid_idx - 1;
        }
    }

    return false;
}
