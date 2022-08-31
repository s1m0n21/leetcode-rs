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
