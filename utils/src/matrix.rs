pub fn transpose_square<T: Clone>(matrix: &mut Vec<Vec<T>>) {
    for i in 0..matrix.len() {
        for j in i + 1..matrix.len() {
            let tmp = matrix[i][j].clone();
            matrix[i][j] = matrix[j][i].clone();
            matrix[j][i] = tmp;
        }
    }
}

pub fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = vec![vec![matrix[0][0].clone(); matrix.len()]; matrix[0].len()];
    for (i, row) in matrix.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            result[j][i] = element.clone();
        }
    }
    result
}

pub fn mirror_in_place<T>(matrix: &mut [Vec<T>]) {
    matrix.iter_mut().for_each(|row| row.reverse());
}

pub fn rotate_left<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let w = matrix[0].len();
    let mut result = vec![vec![matrix[0][0].clone(); matrix.len()]; w];
    for (i, row) in matrix.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            result[w - j - 1][i] = element.clone();
        }
    }
    result
}

pub fn rotate_right<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let h = matrix.len();
    let mut result = vec![vec![matrix[0][0].clone(); h]; matrix[0].len()];
    for (i, row) in matrix.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            result[j][h - i - 1] = element.clone();
        }
    }
    result
}
