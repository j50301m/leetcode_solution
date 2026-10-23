struct Solution {}

impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let converted = Self::covert_to_patterns(&nums);
        let longest_borders = Self::build_longest_borders(&pattern);
        let mut cnt = 0;
        let mut border = 0;
        for i in 0..converted.len() {
            while border > 0 && converted[i] != pattern[border] {
                border = longest_borders[border - 1];
            }

            if converted[i] == pattern[border] {
                border += 1;
            }

            if border == pattern.len() {
                cnt += 1;
                border = longest_borders[border - 1];
            }
        }

        cnt
    }

    fn covert_to_patterns(nums: &Vec<i32>) -> Vec<i32> {
        let mut new_nums = Vec::with_capacity(nums.len() - 1);

        for i in 1..nums.len() {
            let converted = match nums[i].cmp(&nums[i - 1]) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            new_nums.push(converted)
        }
        new_nums
    }

    fn build_longest_borders(nums: &Vec<i32>) -> Vec<usize> {
        let mut longest_borders = vec![0; nums.len()];
        let mut border = 0;
        for i in 1..nums.len() {
            while border > 0 && nums[i] != nums[border] {
                border = longest_borders[border - 1];
            }

            if nums[border] == nums[i] {
                border += 1;
            }

            longest_borders[i] = border;
        }

        longest_borders
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::count_matching_subarrays(vec![1, 2, 3, 4, 5, 6], vec![1, 1]),
            4
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::count_matching_subarrays(vec![1, 4, 4, 1, 3, 5, 5, 3], vec![1, 0, -1]),
            2
        );
    }

    #[test]
    fn overlapping_matches() {
        assert_eq!(
            Solution::count_matching_subarrays(vec![7, 7, 7, 7, 7], vec![0, 0]),
            3
        );
    }

    #[test]
    fn no_match() {
        assert_eq!(
            Solution::count_matching_subarrays(vec![3, 2, 1], vec![1]),
            0
        );
    }
}
