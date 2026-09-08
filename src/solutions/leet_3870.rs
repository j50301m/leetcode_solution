struct Solution {}

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        let divisor = n / 1000;
        let remainder = n % 1000;

        (divisor * 1000 + remainder - 999).max(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::count_commas(1002), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::count_commas(998), 0);
    }
}
