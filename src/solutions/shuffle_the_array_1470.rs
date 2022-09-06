// PROBLEM: https://leetcode.cn/problems/shuffle-the-array
// DATE:    2022/08/29 23:25:45 +0800

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    for i in 0..n {
        ans.push(nums[i as usize]);
        ans.push(nums[(i + n) as usize]);
    }

    ans
}

#[cfg(test)]
use crate::utils::test::TestCase;

#[test]
fn test_solution() {
    let tests = TestCase::new(|p: (Vec<i32>, i32)| shuffle(p.0, p.1));

    tests.run_test((vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
    tests.run_test((vec![1, 2, 3, 4, 4, 3, 2, 1], 4), vec![1, 4, 2, 3, 3, 2, 4, 1]);
    tests.run_test((vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
}
