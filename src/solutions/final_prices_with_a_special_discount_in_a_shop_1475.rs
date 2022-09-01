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
