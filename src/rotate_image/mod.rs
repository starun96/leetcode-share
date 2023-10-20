pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let s = matrix.len();
    for i in 0..s / 2 {
        for j in i..s - 1 - i {
            let mut saved = matrix[i][j];
            let (mut x, mut y) = (i, j);
            for _ in 0..4 {
                let (xp, yp) = (y, s - 1 - x);
                std::mem::swap(&mut saved, &mut matrix[xp][yp]);
                (x, y) = (xp, yp);
            }
        }
    }
}
