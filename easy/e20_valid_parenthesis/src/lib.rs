#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        let map = |ch| match ch {
            ')' => '(',
            '}' => '{',
            ']' => '[',
            _ => unreachable!(),
        };

        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => stack.push(ch),
                ')' | '}' | ']' if stack.is_empty() => return false,
                ')' | '}' | ']' if *stack.last().unwrap() == map(ch) => stack.truncate(stack.len() - 1),
                _ => return false,
            };
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert!(Solution::is_valid(String::from("()[]{}")))
    }

    #[test]
    fn case_2() {
        assert!(Solution::is_valid(String::from("()")))
    }

    #[test]
    fn case_3() {
        assert!(!Solution::is_valid(String::from("{[]}")))
    }
}
