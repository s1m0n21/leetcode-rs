// PROBLEM: https://leetcode.cn/problems/maximum-product-of-two-elements-in-an-array
// DATE:    2022/08/26 01:58:49 +0800

pub fn max_product(nums: Vec<i32>) -> i32 {
    assert!(nums.len() >= 2);

    let mut nums = nums;
    nums.sort();

    (nums[nums.len()-1]-1) * (nums[nums.len()-2]-1)
}

#[cfg(test)]
use crate::utils::test::TestCase;

#[test]
fn test_solution() {
    let tests = TestCase::new(max_product);

    tests.run_test(vec![3, 4, 5, 2], 12);
    tests.run_test(vec![1, 5, 4, 5], 16);
    tests.run_test(vec![3, 7], 12);
}
