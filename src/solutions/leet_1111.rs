struct Solution {}

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        const A: i32 = 0;
        const B: i32 = 1;

        let bytes = seq.as_bytes();
        // let mut curr_open = None;
        let mut a_cnt = 0;
        let mut b_cnt = 0;

        let mut result = vec![0; bytes.len()];
        for (i, &byte) in bytes.iter().enumerate() {
            let char = byte as char;
            if char == '(' {
                if a_cnt <= b_cnt {
                    a_cnt += 1;
                    result[i] = A;
                } else {
                    b_cnt += 1;
                    result[i] = B;
                }
            } else {
                if b_cnt >= a_cnt {
                    b_cnt -= 1;
                    result[i] = B;
                } else {
                    a_cnt -= 1;
                    result[i] = A;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 只看 ans[i] == group 的字元，確認是合法括號字串，回傳它的 depth
    fn depth_of(seq: &str, ans: &[i32], group: i32) -> i32 {
        let mut open = 0;
        let mut max = 0;
        for (c, &g) in seq.chars().zip(ans) {
            if g != group {
                continue;
            }
            if c == '(' {
                open += 1;
                max = max.max(open);
            } else {
                open -= 1;
            }
            assert!(open >= 0, "group {group} invalid: {seq} -> {ans:?}");
        }
        assert_eq!(open, 0, "group {group} not closed: {seq} -> {ans:?}");
        max
    }

    fn check(seq: &str) {
        let ans = Solution::max_depth_after_split(seq.to_string());
        assert_eq!(ans.len(), seq.len());
        assert!(ans.iter().all(|&g| g == 0 || g == 1), "{ans:?}");

        let original = depth_of(seq, &vec![0; seq.len()], 0);
        let best = depth_of(seq, &ans, 0).max(depth_of(seq, &ans, 1));
        assert_eq!(best, (original + 1) / 2, "{seq} -> {ans:?}");
    }

    #[test]
    fn example1() {
        check("(()())");
    }

    #[test]
    fn example2() {
        check("()(())()");
    }

    #[test]
    fn deep_nesting() {
        check("(((())))");
        check("((((()))))");
    }

    #[test]
    fn mixed() {
        check("()");
        check("()()()");
        check("((()))()(())");
        check("(()(()))(((())))");
    }
}
