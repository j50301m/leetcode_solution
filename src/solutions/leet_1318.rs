struct Solution {}

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut fold = (a | b) ^ c;
        let mut cnt = 0;
        while fold > 0 {
            fold &= fold - 1;
            cnt += 1;
        }

        let mut union = (a & b) & !c;
        while union > 0 {
            union &= union - 1;
            cnt += 1;
        }

        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_flips(2, 6, 5), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_flips(4, 2, 7), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::min_flips(1, 2, 3), 0);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::min_flips(7, 7, 7), 0);
    }
}
