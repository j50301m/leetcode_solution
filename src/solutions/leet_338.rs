struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![0; (n + 1) as usize];
        for i in 1..=n {
            let mut cnt = 0;
            let mut num = i;
            while num > 0 {
                num &= num - 1;
                cnt += 1;
            }
            result[i as usize] = cnt;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
