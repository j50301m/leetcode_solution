struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut small_tail = vec![i32::MAX; n];
        for i in 0..n {
            let num = nums[i];
            let pos = small_tail.partition_point(|&x| x < num);
            small_tail[pos] = num;
        }
        let len = small_tail.partition_point(|&x| x < i32::MAX);
        len as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::length_of_lis(vec![3, 1, 4, 2, 5]), 3);
    }
}
