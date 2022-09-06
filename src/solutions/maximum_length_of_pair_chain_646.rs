// PROBLEM: https://leetcode.cn/problems/maximum-length-of-pair-chain
// DATE:    2022/09/03 15:39:41 +0800

pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
    pairs.sort_by_key(|x| x[1]);

    let mut max = 1;
    let mut right = pairs[0][1];

    for i in 1..pairs.len() {
        if pairs[i][0] > right {
            max += 1;
            right = pairs[i][1];
        }
    }

    max
}

#[cfg(test)]
use crate::utils::test::TestCase;

#[test]
fn test_maximum_length_of_pair_chain_646() {
    let tests = TestCase::new(find_longest_chain);

    tests.run_test(vec![vec![1, 2], vec![2, 3], vec![3, 4]], 2);
}
