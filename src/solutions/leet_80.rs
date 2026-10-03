struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut appear_cnt = 1;
        let mut idx = 1;
        for i in 1..nums.len() {
            if nums[i - 1] != nums[i] {
                nums[idx] = nums[i];
                appear_cnt = 1;
                idx += 1;
                continue;
            }
            appear_cnt += 1;
            if appear_cnt <= 2 {
                nums[idx] = nums[i];
                idx += 1;
            }
        }
        idx as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let mut vec = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut vec), 5);
    }

    #[test]
    fn case2() {
        let mut vec = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut vec), 7);
        assert_eq!(vec, vec![0, 0, 1, 1, 2, 3, 3, 3, 3]);
    }
}
