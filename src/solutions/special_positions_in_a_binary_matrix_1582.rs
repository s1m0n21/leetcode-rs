// PROBLEM: https://leetcode.cn/problems/special-positions-in-a-binary-matrix
// DATE:    2022/09/04 01:37:25 +0800

pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let mut special = 0_i32;
    let (m, n) = (mat.len(), mat[0].len());
    let mut rows = vec![0; m];
    let mut cols = vec![0; n];

    for i in 0..m {
        for j in 0..n {
            rows[i] += mat[i][j];
            cols[j] += mat[i][j];
        }
    }

    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 1 && rows[i] == 1 && cols[j] == 1 {
                special += 1;
            }
        }
    }

    special
}
