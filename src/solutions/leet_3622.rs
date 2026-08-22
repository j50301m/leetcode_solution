struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut num = n;
        let mut sum = 0;
        let mut product = 1;

        while num > 0 {
            let curr = num % 10;
            sum += curr;
            product *= curr;
            num /= 10;
        }

        n % (sum + product) == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::check_divisibility(99), true);
    }
}
