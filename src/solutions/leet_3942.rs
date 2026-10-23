struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut desc_cnt = 0;
        let mut pos0 = 0;
        let mut posn = 0;
        for i in 0..nums.len() {
            if nums[i] > nums[(i + 1) % nums.len()] {
                desc_cnt += 1;
            }
            if nums[i] == 0 {
                pos0 = i;
            }
            if nums[i] == (nums.len() - 1) as i32 {
                posn = i;
            }
        }

        if !(desc_cnt <= 1 || nums.len() - desc_cnt <= 1) {
            return -1;
        }

        if pos0 == 0 && posn == nums.len() - 1 {
            return 0;
        }

        if desc_cnt <= 1 {
            let rotate = pos0;
            let reserve_twice = (nums.len() - 1 - posn) + 2;
            return rotate.min(reserve_twice) as i32;
        } else {
            let reserve_first = posn + 1;
            let rotate_first = 1 + (nums.len() - 1 - pos0);

            return reserve_first.min(rotate_first) as i32;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_operations(vec![0, 2, 1]), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_operations(vec![1, 0, 2]), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::min_operations(vec![1, 2, 0]), 2)
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3, 4, 0]), 3);
    }
}
