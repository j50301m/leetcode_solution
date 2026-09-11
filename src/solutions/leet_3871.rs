struct Solution {}

impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        // 分層計數：每個 10^(3j) 門檻貢獻「1..n 裡 >= 門檻的數字個數」
        // ponytail: 上限寫死 6，因為 i64 裝不下 10^21，不需要動態擴張
        (1..=6).map(|j| (n - 10i64.pow(3 * j) + 1).max(0)).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::count_commas(1004590), 1008182);
    }

    #[test]
    fn no_comma() {
        assert_eq!(Solution::count_commas(1), 0);
        assert_eq!(Solution::count_commas(998), 0);
    }

    #[test]
    fn one_layer() {
        assert_eq!(Solution::count_commas(1002), 3);
    }

    #[test]
    fn two_layers() {
        assert_eq!(Solution::count_commas(1_000_000), 999_002);
    }
}
