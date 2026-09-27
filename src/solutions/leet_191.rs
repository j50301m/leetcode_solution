struct Solution {}

impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut cnt = 0;
        while n > 0 {
            n = n & (n - 1);
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
        assert_eq!(Solution::hamming_weight(11), 3);
    }
}
