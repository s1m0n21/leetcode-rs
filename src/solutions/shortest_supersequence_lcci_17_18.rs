// PROBLEM: https://leetcode.cn/problems/shortest-supersequence-lcci
// DATE:    2022/08/27 00:33:31 +0800

use std::collections::HashMap;

pub fn shortest_seq(big: Vec<i32>, small: Vec<i32>) -> Vec<i32> {
    if big.len() == 0 { return vec![] }

    let mut ans :Vec<i32> = vec![];
    let mut needs = small.len();
    let mut cnt :HashMap<i32, i32> = HashMap::new();

    for n in small.iter() {
        cnt.insert(n.clone(), 0);
    }

    if cnt.get(&big[0]) != None {
        needs -= 1;
        cnt.insert(big[0], 1);
    }

    let (mut l, mut r) = (0, 0);
    while r < big.len() {
        if needs > 0 {
            r += 1;
            if r < big.len() {
                if let Some(n) = cnt.get(&big[r]) {
                    if n.clone() == 0 { needs -= 1 }
                    cnt.insert(big[r], n+1);
                }
            }
        } else {
            if let Some(n) = cnt.get(&big[l]) {
                if n.clone() == 1 { needs += 1 }
                cnt.insert(big[l], n-1);
            }
            l += 1;
        }

        if needs == 0 {
            if ans.len() == 2 {
                if ((r - l) as i32) < (ans[1] - ans[0]) {
                    ans[0] = l as i32;
                    ans[1] = r as i32;
                }
            } else {
                ans.push(l as i32);
                ans.push(r as i32);
            }
        }
    }

    ans
}

#[cfg(test)]
use crate::utils::test::TestCase;

#[test]
fn test_solution() {
    let tests = TestCase::new(|p: (Vec<i32>, Vec<i32>)| shortest_seq(p.0, p.1));

    tests.run_test((vec![7, 5, 9, 0, 2, 1, 3, 5, 7, 9, 1, 1, 5, 8, 8, 9, 7], vec![1, 5, 9]), vec![7, 10]);
    tests.run_test((vec![1, 2, 3], vec![4]), vec![]);
    tests.run_test((vec![], vec![4]), vec![]);
}
