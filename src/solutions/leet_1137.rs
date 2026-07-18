struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        let mut result = vec![0; n + 1];
        if n == 0 {
            return 0;
        } else if n == 1 || n == 2 {
            return 1;
        }
        result[0] = 0;
        result[1] = 1;
        result[2] = 1;

        for i in 3..=n {
            result[i] = result[i - 1] + result[i - 2] + result[i - 3];
        }

        result[n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let n = 4;
        let result = Solution::tribonacci(n);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_2() {
        let n = 25;
        let result = Solution::tribonacci(n);
        assert_eq!(result, 1389537);
    }
}
