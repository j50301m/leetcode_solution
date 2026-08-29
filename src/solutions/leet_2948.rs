struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut indices: Vec<usize> = (0..nums.len()).collect();
        indices.sort_by_key(|&i| nums[i]);

        let mut groups: Vec<Vec<usize>> = Vec::new();
        let mut left = 0;
        while left < nums.len() {
            let mut right = left;
            while right + 1 < nums.len() && nums[indices[right + 1]] - nums[indices[right]] <= limit
            {
                right += 1;
            }

            groups.push(indices[left..=right].to_vec());
            left = right + 1;
        }

        let mut ans = vec![0; nums.len()];
        for group in groups.iter() {
            let mut sorted = group.clone();
            sorted.sort();

            for k in 0..group.len() {
                ans[sorted[k]] = nums[group[k]];
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2),
            vec![1, 3, 5, 8, 9]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3),
            vec![1, 6, 7, 18, 1, 2]
        );
    }

    // 任兩個差都 > limit，一次都換不動
    #[test]
    fn case3() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 7, 28, 19, 10], 3),
            vec![1, 7, 28, 19, 10]
        );
    }

    // 差「剛好等於」limit 也算換得動：1-4-7 串成同一組
    #[test]
    fn case4() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![4, 1, 7], 3),
            vec![1, 4, 7]
        );
    }

    // limit = 0：只有相同值能互換，等於原地不動
    #[test]
    fn case5() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![3, 1, 3, 1], 0),
            vec![3, 1, 3, 1]
        );
    }

    // 兩個群組的位置是交錯的，不是連續區間
    #[test]
    fn case6() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![11, 2, 10, 1], 1),
            vec![10, 1, 11, 2]
        );
    }

    // 全部串成一組，直接整個排序
    #[test]
    fn case7() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![5, 4, 3, 2, 1], 1),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn case8() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![5], 0),
            vec![5]
        );
    }
}
