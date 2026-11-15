struct Solution {}

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1000000007;

        let n = arr.len();
        let mut odd = vec![0; n + 1];
        let mut even = vec![0; n + 1];
        let mut sum = 0;
        for i in 0..n {
            let num = arr[i];
            if num % 2 == 1 {
                odd[i + 1] = (even[i] + 1) % MOD;
                even[i + 1] = odd[i];
            } else {
                odd[i + 1] = odd[i];
                even[i + 1] = (even[i] + 1) % MOD;
            }
            sum = (sum + odd[i + 1]) % MOD;
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);
    }
    #[test]
    fn case3() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
    }
}
