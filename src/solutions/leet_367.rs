struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let mut left = 1i64;
        let mut right = num;

        //  idx 1,2,3,4,5,6...10
        //  taget 16

        // 1,2. target =1
        // l = 0, r= 1, m =0
        while left < right {
            let mid = left + (right - left) / 2;

            if mid > num || mid * mid >= num {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left * left == num
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::is_perfect_square(13), false);
        assert_eq!(Solution::is_perfect_square(i32::MAX), false);
    }
}
