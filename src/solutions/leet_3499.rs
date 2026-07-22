struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let n = s.len();
        let mut cnt_1 = 0;
        let mut zero_blocks = Vec::new();
        let mut i = 0;
        let bytes = s.as_bytes();
        while i < n {
            let start = i;
            while i < n && bytes[i] == bytes[start] {
                i += 1;
            }
            if bytes[start] == b'1' {
                cnt_1 += i - start;
            } else {
                zero_blocks.push(i - start);
            }
        }

        if zero_blocks.len() < 2 {
            return cnt_1 as i32;
        }

        let mut cnt_0 = 0;
        for i in 1..zero_blocks.len() {
            cnt_0 = cnt_0.max(zero_blocks[i] + zero_blocks[i - 1]);
        }

        (cnt_0 + cnt_1) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let s = String::from("01");
        let result = Solution::max_active_sections_after_trade(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_2() {
        let s = String::from("0100");
        let result = Solution::max_active_sections_after_trade(s);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_3() {
        let s = String::from("1000100");
        let result = Solution::max_active_sections_after_trade(s);
        assert_eq!(result, 7);
    }

    #[test]
    fn case_4() {
        let s = String::from("01010");
        let result = Solution::max_active_sections_after_trade(s);
        assert_eq!(result, 4);
    }
}
