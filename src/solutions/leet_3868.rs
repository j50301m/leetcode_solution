struct Solution {}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        if s.len() < 2 {
            return 0;
        }

        let bytes = s.as_bytes();
        if s.len() == 2 {
            let curr = (bytes[1] - b'a') as i32;
            let pre = (bytes[0] - b'a') as i32;
            if curr - pre < 0 {
                return -1;
            }
            return 0;
        }

        let mut is_sorted = true;
        let mut min = bytes[0];
        let mut max = bytes[0];
        for i in 1..s.len() {
            let curr = bytes[i];
            let pre = bytes[i - 1];
            if is_sorted && curr < pre {
                is_sorted = false;
            }

            if curr < min {
                min = curr;
            }
            if curr > max {
                max = curr;
            }
        }

        if is_sorted {
            return 0;
        }
        if bytes[0] == min || bytes[s.len() - 1] == max {
            return 1;
        }

        let middle = &bytes[1..s.len() - 1];
        if bytes[0] == max
            && bytes[s.len() - 1] == min
            && !middle.contains(&min)
            && !middle.contains(&max)
        {
            return 3;
        }

        2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_operations("dog".to_string()), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_operations("card".to_string()), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::min_operations("card".to_string()), 2);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::min_operations("gf".to_string()), -1);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::min_operations("cab".to_string()), 2);
    }

    #[test]
    fn case6() {
        assert_eq!(Solution::min_operations("edbc".to_string()), 2);
    }

    #[test]
    fn case7() {
        assert_eq!(Solution::min_operations("dada".to_string()), 2);
    }

    #[test]
    fn case8() {
        assert_eq!(Solution::min_operations("a".to_string()), 0);
    }

    #[test]
    fn case9() {
        assert_eq!(Solution::min_operations("dcba".to_string()), 3);
    }
}
