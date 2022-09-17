// PROBLEM: https://leetcode.cn/problems/largest-substring-between-two-equal-characters
// DATE:    2022/09/17 04:02:19 +0800

use std::cmp::max;
use std::collections::HashMap;

pub fn max_length_between_equal_characters(s: String) -> i32 {
    let mut map: HashMap<u8, usize> = HashMap::new();
    let mut max_length = -1;

    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if let Some(n) = map.get(&b) {
            max_length = max(max_length, (i - n) as i32);
        } else {
            map.insert(b, i + 1);
        }
    }

    max_length
}

#[cfg(test)]
use crate::utils::test::TestCase;

#[test]
fn test_solution() {
    let tests = TestCase::new(max_length_between_equal_characters);

    tests.run_test("aa".to_string(), 0);
    tests.run_test("abca".to_string(), 2);
    tests.run_test("cbzxy".to_string(), -1);
    tests.run_test("cabbac".to_string(), 4);
}