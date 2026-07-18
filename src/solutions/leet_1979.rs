pub struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut min = 1001;
        let mut max = 0;
        for &ele in nums.iter() {
            if ele > max {
                max = ele;
            }
            if ele < min {
                min = ele;
            }
        }

        Self::gcd(min, max)
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
        let nums = vec![2, 5, 6, 9, 10];
        let result = Solution::find_gcd(nums);
        println!("{:?}", result);
    }
}
