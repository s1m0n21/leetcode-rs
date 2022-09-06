// PROBLEM: https://leetcode.cn/problems/final-prices-with-a-special-discount-in-a-shop
// DATE:    2022/09/01 13:44:32 +0800

pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut res = prices.clone();
    let mut stack :Vec<i32> = vec![];

    for i in (0..prices.len()).rev() {
        while !stack.is_empty() && stack[stack.len()-1] > prices[i] {
            stack.pop();
        }

        if !stack.is_empty() {
            res[i] = prices[i] - stack[stack.len()-1];
        }
        stack.push(prices[i]);
    }

    res
}

#[cfg(test)]
use crate::utils::test::TestCase;

#[test]
fn test_solution() {
    let tests = TestCase::new(final_prices);

    tests.run_test(vec![8, 4, 6, 2, 3], vec![4, 2, 4, 2, 3]);
    tests.run_test(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);
    tests.run_test(vec![10, 1, 1, 6], vec![9, 0, 1, 6]);
}
