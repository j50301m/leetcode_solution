struct Solution {}

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut result = Vec::new();
        let (rm_left, rm_right) = Self::scan_need_remove(bytes);
        let mut memo = Vec::new();
        Self::dfs(
            bytes,
            0,
            false,
            0,
            rm_left,
            rm_right,
            &mut memo,
            &mut result,
        );
        result
    }

    /// Returning (i32,i32) means (need_remove_left_count, need_remove_right_count)
    fn scan_need_remove(bytes: &[u8]) -> (i32, i32) {
        let mut rm_right = 0;
        let mut open = 0;
        for &b in bytes {
            match b {
                b'(' => {
                    open += 1;
                }
                b')' => {
                    if open == 0 {
                        rm_right += 1;
                        continue;
                    }
                    open -= 1;
                }
                _ => {}
            }
        }
        (open, rm_right)
    }

    fn dfs(
        bytes: &[u8],
        idx: usize,
        prev_kept: bool,
        open: i32,
        rm_left: i32,
        rm_right: i32,
        memo: &mut Vec<u8>,
        result: &mut Vec<String>,
    ) {
        if idx == bytes.len() {
            if open == 0 && rm_left == 0 && rm_right == 0 {
                let s = String::from_utf8(memo.clone()).unwrap();
                result.push(s);
            }
            return;
        }

        if rm_left + rm_right > (bytes.len() - idx) as i32 {
            return;
        }

        let b = bytes[idx];
        let can_delete = idx == 0 || bytes[idx - 1] != b || !prev_kept;
        match b {
            b'(' => {
                // Delete curr char
                if rm_left > 0 && can_delete {
                    Self::dfs(
                        bytes,
                        idx + 1,
                        false,
                        open,
                        rm_left - 1,
                        rm_right,
                        memo,
                        result,
                    );
                }

                memo.push(b);
                Self::dfs(
                    bytes,
                    idx + 1,
                    true,
                    open + 1,
                    rm_left,
                    rm_right,
                    memo,
                    result,
                );
                memo.pop();
            }
            b')' => {
                if rm_right > 0 && can_delete {
                    Self::dfs(
                        bytes,
                        idx + 1,
                        false,
                        open,
                        rm_left,
                        rm_right - 1,
                        memo,
                        result,
                    );
                }

                if open > 0 {
                    memo.push(b);
                    Self::dfs(
                        bytes,
                        idx + 1,
                        true,
                        open - 1,
                        rm_left,
                        rm_right,
                        memo,
                        result,
                    );
                    memo.pop();
                }
            }
            _ => {
                memo.push(b);
                Self::dfs(bytes, idx + 1, false, open, rm_left, rm_right, memo, result);
                memo.pop();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(s: &str) -> Vec<String> {
        let mut v = Solution::remove_invalid_parentheses(s.to_string());
        v.sort();
        v
    }

    fn expect(v: &[&str]) -> Vec<String> {
        let mut v: Vec<String> = v.iter().map(|s| s.to_string()).collect();
        v.sort();
        v
    }

    #[test]
    fn case1() {
        assert_eq!(run("()())()"), expect(&["(())()", "()()()"]));
    }

    #[test]
    fn case2_with_letters() {
        assert_eq!(run("(a)())()"), expect(&["(a())()", "(a)()()"]));
    }

    #[test]
    fn case3_all_removed() {
        assert_eq!(run(")("), expect(&[""]));
    }

    #[test]
    fn already_valid() {
        assert_eq!(run("(a)(b)"), expect(&["(a)(b)"]));
    }

    #[test]
    fn only_letters() {
        assert_eq!(run("abc"), expect(&["abc"]));
    }

    #[test]
    fn extra_left() {
        assert_eq!(run("x("), expect(&["x"]));
    }

    #[test]
    fn no_duplicates() {
        assert_eq!(run("())"), expect(&["()"]));
    }

    #[test]
    fn no_duplicates_left() {
        assert_eq!(run("(()"), expect(&["()"]));
    }
}
