// PROBLEM: https://leetcode.cn/problems/make-two-arrays-equal-by-reversing-sub-arrays
// DATE:    2022/08/24 02:32:31 +0800

pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    let mut target = target;
    let mut arr = arr;

    target.sort();
    arr.sort();

    for i in 0..target.len() {
        if target[i] != arr[i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
use crate::utils::test::TestCase;

#[test]
fn test_solution() {
    let tests = TestCase::new(|p: (Vec<i32>, Vec<i32>)| can_be_equal(p.0, p.1));

    tests.run_test((vec![1, 2, 3, 4], vec![2, 4, 1, 3]), true);
    tests.run_test((vec![7], vec![7]), true);
    tests.run_test((vec![3, 7, 9], vec![3, 7, 11]), false);
}
