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
