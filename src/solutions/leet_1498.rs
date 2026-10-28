struct Solution {}

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let mut power_of_two = vec![1i64; nums.len()];
        for i in 1..nums.len() {
            power_of_two[i] = power_of_two[i - 1] * 2 % MOD;
        }
        nums.sort();
        let mut sum = 0;
        for i in 0..nums.len() {
            let need = target - nums[i];
            let pos = nums[i..].partition_point(|x| *x <= need);
            if pos == 0 {
                continue;
            }

            sum = (sum + power_of_two[pos - 1]) % MOD;
        }

        sum as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::num_subseq(vec![3, 5, 6, 7], 9), 4);
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::num_subseq(
                vec![
                    14, 4, 6, 6, 20, 8, 5, 6, 8, 12, 6, 10, 14, 9, 17, 16, 9, 7, 14, 11, 14, 15,
                    13, 11, 10, 18, 13, 17, 17, 14, 17, 7, 9, 5, 10, 13, 8, 5, 18, 20, 7, 5, 5, 15,
                    19, 14
                ],
                22
            ),
            272187084
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::num_subseq(
                vec![
                    27, 21, 14, 2, 15, 1, 19, 8, 12, 24, 21, 8, 12, 10, 11, 30, 15, 18, 28, 14, 26,
                    9, 2, 24, 23, 11, 7, 12, 9, 17, 30, 9, 28, 2, 14, 22, 19, 19, 27, 6, 15, 12,
                    29, 2, 30, 11, 20, 30, 21, 20, 2, 22, 6, 14, 13, 19, 21, 10, 18, 30, 2, 20, 28,
                    22
                ],
                31
            ),
            688052206
        );
    }
}
