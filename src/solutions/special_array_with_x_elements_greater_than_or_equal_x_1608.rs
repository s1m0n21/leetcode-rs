// PROBLEM: https://leetcode.cn/problems/special-array-with-x-elements-greater-than-or-equal-x
// DATE:    2022/09/12 00:52:33 +0800

pub fn special_array(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums.sort_by(|a, b| b.cmp(a));
    for i in 1..n+1 {
        if nums[i-1] >= i as i32 && (i == n || nums[i] < i as i32) {
            return i as i32
        }
    }

    -1
}

#[cfg(test)]
use crate::utils::test::TestCase;

#[test]
fn test_solution() {
    let tests = TestCase::new(special_array);

    tests.run_test(vec![5, 3], 2);
    tests.run_test(vec![0, 0], -1);
    tests.run_test(vec![0, 4, 3, 0, 4], 3);
    tests.run_test(vec![3, 6, 7, 7, 0], -1);
}