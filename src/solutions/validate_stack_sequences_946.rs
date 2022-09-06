// PROBLEM: https://leetcode.cn/problems/validate-stack-sequences
// DATE:    2022/08/31 21:30:36 +0800

pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack :Vec<i32> = vec![];
    let mut i = 0_usize;
    let mut j = 0_usize;

    while i <= pushed.len() {
        if stack.len() > 0 && stack[stack.len()-1] == popped[j] {
            stack.pop();
            j += 1;
        } else {
            if i < pushed.len() {
                stack.push(pushed[i]);
            }
            i += 1;
        }
    }

    j == popped.len()
}

#[cfg(test)]
use crate::utils::test::TestCase;

#[test]
fn test_solution() {
    let tests = TestCase::new(|p: (Vec<i32>, Vec<i32>)| validate_stack_sequences(p.0, p.1));

    tests.run_test((vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]), true);
    tests.run_test((vec![1, 2, 3, 4, 5], vec![4, 5, 3, 1, 2]), false);
}
