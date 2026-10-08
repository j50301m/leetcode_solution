struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let last_ele = nums.last().unwrap();
        let pos = nums.partition_point(|&x| x > *last_ele);
        nums[pos]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }
}
