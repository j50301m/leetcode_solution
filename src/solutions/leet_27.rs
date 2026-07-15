pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }

        k as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums = vec![3, 2, 2, 3];
        let k = Solution::remove_element(&mut nums, 3);
        assert_eq!(k, 2);
        assert_eq!(&nums[..2], &[2, 2]);
    }

    #[test]
    fn case_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let k = Solution::remove_element(&mut nums, 2);
        assert_eq!(k, 5);
        assert_eq!(&nums[..5], &[0, 1, 3, 0, 4]);
    }
}
