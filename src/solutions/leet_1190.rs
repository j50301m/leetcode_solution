use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let bytes = s.as_bytes();
        let mut i = 0;

        fn parse(bytes: &[u8], idx: &mut usize) -> VecDeque<char> {
            let mut queue = VecDeque::new();
            while *idx < bytes.len() {
                let b = bytes[*idx];
                *idx += 1;
                match b {
                    b'(' => {
                        let val = parse(bytes, idx);
                        for v in val {
                            queue.push_front(v);
                        }
                    }
                    b')' => {
                        break;
                    }
                    _ => queue.push_front(b as char),
                }
            }

            queue
        }

        let mut ans = String::new();
        let mut queue = parse(bytes, &mut i);
        while let Some(c) = queue.pop_back() {
            ans.push(c);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::reverse_parentheses("(ed(et(oc))el)".to_string()),
            "leetcode".to_string()
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::reverse_parentheses("(u(love)i)".to_string(),),
            "iloveu".to_string()
        )
    }
}
