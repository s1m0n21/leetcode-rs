// PROBLEM: https://leetcode.cn/problems/maximum-product-of-two-elements-in-an-array
// DATE:    2022/08/26 01:58:49 +0800

pub fn max_product(nums: Vec<i32>) -> i32 {
    assert!(nums.len() >= 2);

    let mut nums = nums;
    nums.sort();

    (nums[nums.len()-1]-1) * (nums[nums.len()-2]-1)
}
