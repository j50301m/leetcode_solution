pub struct Solution;

impl Solution {
    pub fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for start in 0..nums.len() {
            let mut curr_gcd = 0;
            for end in start..nums.len() {
                let gcd = Self::gcd(curr_gcd, nums[end]);
                if gcd == k {
                    count += 1;
                } else if gcd < k {
                    break;
                }
                curr_gcd = gcd;
            }
        }
        count
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let tmp = a % b;
            a = b;
            b = tmp;
        }
        a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![9, 3, 1, 2, 6, 3];
        let k = 3;

        let result = Solution::subarray_gcd(nums, k);
        println!("{:?}", result);
    }

    #[test]
    fn case_2() {
        let nums = vec![4];
        let k = 7;

        let result = Solution::subarray_gcd(nums, k);
        println!("{:?}", result);
    }
}
