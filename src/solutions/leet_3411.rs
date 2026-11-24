struct Solution {}

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let primes = Self::prime_up_to(10);
        let prime_2_idx = Self::prime_2_idx(&primes, 10);

        let mut left = 0usize;
        let mut max_len = 2usize;
        let mut count = vec![0; primes.len()];
        for right in 0..n {
            let right_factors = Self::find_prime(&prime_2_idx, &primes, nums[right] as usize);
            for prime_idx in 0..primes.len() {
                count[prime_idx] += right_factors[prime_idx];
            }

            while count.iter().any(|&prime_count| prime_count >= 2) {
                let left_factors = Self::find_prime(&prime_2_idx, &primes, nums[left] as usize);
                for prime_idx in 0..primes.len() {
                    count[prime_idx] -= left_factors[prime_idx];
                }
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }

    fn find_prime(prime_2_idx: &[usize], primes: &[usize], a: usize) -> Vec<i32> {
        let mut count = vec![0; primes.len()];
        for &prime in primes {
            if a % prime == 0 {
                count[prime_2_idx[prime]] += 1;
            }
        }
        count
    }

    fn prime_2_idx(primes: &[usize], limit: usize) -> Vec<usize> {
        let mut prime_2_idx = vec![0usize; limit + 1];
        let mut i = 0usize;
        for &prime in primes {
            prime_2_idx[prime] = i;
            i += 1;
        }
        prime_2_idx
    }

    fn prime_up_to(limit: usize) -> Vec<usize> {
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        let mut num = 2;
        while num * num <= limit {
            if is_prime[num] {
                let mut multiple = num * num;
                while multiple <= limit {
                    is_prime[multiple] = false;
                    multiple += num;
                }
            }
            num += 1;
        }
        is_prime
            .iter()
            .enumerate()
            .filter_map(|(num, &is_prime)| if is_prime { Some(num) } else { None })
            .collect()
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = a % b;
            a = b;
            b = temp;
        }
        a
    }

    fn lcm(a: i32, b: i32) -> i32 {
        a / Self::gcd(a, b) * b
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_length(vec![1, 2, 1, 2, 1, 1, 1]), 5);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::max_length(vec![2, 3, 4, 5, 6]), 3);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::max_length(vec![2, 6]), 2);
    }
}
