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