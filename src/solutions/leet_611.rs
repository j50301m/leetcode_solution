struct Solution {}

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        nums.sort_unstable_by(|a, b| b.cmp(a));

        let mut cnt = 0;
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                let long_side = nums[i];
                let target = long_side - nums[j];

                let pos = nums[j + 1..].partition_point(|&x| x > target);
                cnt += pos;
            }
        }

        cnt as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::triangle_number(vec![4, 2, 3, 4]), 4);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::triangle_number(vec![1]), 0);
    }
}
