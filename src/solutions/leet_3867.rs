struct Solution;

impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let gcd = |mut a: i32, mut b: i32| -> i32 {
            while b != 0 {
                let tmp = a % b;
                a = b;
                b = tmp;
            }
            a
        };

        let mut prefix_gcd = Vec::with_capacity(nums.len());
        let mut max = 0;
        for ele in nums {
            if ele > max {
                max = ele;
                prefix_gcd.push(ele);
                continue;
            }

            prefix_gcd.push(gcd(ele, max));
        }

        prefix_gcd.sort();
        let mut sum: i64 = 0;
        for i in 0..prefix_gcd.len() / 2 {
            let a = prefix_gcd[i];
            let b = prefix_gcd[prefix_gcd.len() - i - 1];
            sum += gcd(a, b) as i64;
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![2, 6, 4];
        let result = Solution::gcd_sum(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_2() {
        let nums = vec![3, 6, 2, 8];
        let result = Solution::gcd_sum(nums);
        assert_eq!(result, 5);
    }
}
