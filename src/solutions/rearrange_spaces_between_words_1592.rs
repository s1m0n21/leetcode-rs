// PROBLEM: https://leetcode.cn/problems/rearrange-spaces-between-words
// DATE:    2022/09/07 03:08:54 +0800

pub fn reorder_spaces(text: String) -> String {
    let words :Vec<&str> = text.split_whitespace().collect();
    let spaces = text.len() - words.iter().map(|x| x.len()).sum::<usize>();
    let pre = spaces.checked_div(words.len() - 1).unwrap_or(0);
    let extra = spaces.checked_rem(words.len() - 1).unwrap_or(spaces);

    let mut ans = if words.len() > 1 {
        words.join(" ".repeat(pre).as_str())
    } else {
        words[0].to_string() + " ".repeat(pre).as_str()
    };

    if extra > 0 {
        ans.push_str(" ".repeat(extra).as_str());
    }

    ans
}

#[cfg(test)]
use crate::utils::test::TestCase;

#[test]
fn test_solution() {
    let tests = TestCase::new(reorder_spaces);

    tests.run_test("  this   is  a sentence ".to_string(), "this   is   a   sentence".to_string());
    tests.run_test(" practice   makes   perfect".to_string(), "practice   makes   perfect ".to_string());
    tests.run_test("hello   world".to_string(), "hello   world".to_string());
    tests.run_test("  walks  udp package   into  bar a".to_string(), "walks  udp  package  into  bar  a ".to_string());
    tests.run_test("a".to_string(), "a".to_string());
    tests.run_test("a b c ".to_string(), "a b c ".to_string());
}