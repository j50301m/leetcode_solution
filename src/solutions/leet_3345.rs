struct Solution;

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        let mut curr = n;
        loop {
            let mut n = curr;
            let mut produvt = 1;
            while n > 0 {
                produvt *= n % 10;
                n /= 10;
            }

            if produvt % t == 0 {
                return curr;
            }
            curr += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::smallest_number(15, 3), 16);
    }
}
