struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut used = 0;
        let mut best = 0;

        for right in 0..nums.len() {
            let curr = nums[right];
            if curr == 0 {
                used += 1;
            }

            while used > k {
                if nums[left] == 0 {
                    used -= 1;
                }
                left += 1;
            }

            best = best.max(right - left + 1);
        }

        best as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
            6
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            ),
            10
        );
    }
}
