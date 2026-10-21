struct Solution {}

impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut start = 0;
        let mut cnt = 0;
        for group in groups.iter() {
            let longest_borders = Self::build_long_border(&group);
            let mut border = 0;
            for i in start..nums.len() {
                while border > 0 && nums[i] != group[border] {
                    border = longest_borders[border - 1];
                }

                if nums[i] == group[border] {
                    border += 1;
                }

                if border == group.len() {
                    start = i + 1;
                    cnt += 1;
                    break;
                }
            }
        }
        cnt == groups.len()
    }

    fn build_long_border(group: &Vec<i32>) -> Vec<usize> {
        let mut longest_borders = vec![0usize; group.len()];
        let mut border = 0;
        for i in 1..group.len() {
            while border > 0 && group[i] != group[border] {
                border = longest_borders[border - 1];
            }

            if group[i] == group[border] {
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
        let groups = vec![vec![1, -1, -1], vec![3, -2, 0]];
        let nums = vec![1, -1, 0, 1, -1, -1, 3, -2, 0];
        assert!(Solution::can_choose(groups, nums));
    }

    #[test]
    fn case2() {
        // 順序不對：[1,2,3,4] 必須在 [10,-2] 之後
        let groups = vec![vec![10, -2], vec![1, 2, 3, 4]];
        let nums = vec![1, 2, 3, 4, 10, -2];
        assert!(!Solution::can_choose(groups, nums));
    }

    #[test]
    fn case3() {
        // 兩個 group 不能共用同一個元素（3 被重疊）
        let groups = vec![vec![1, 2, 3], vec![3, 4]];
        let nums = vec![7, 7, 1, 2, 3, 4, 7, 7];
        assert!(!Solution::can_choose(groups, nums));
    }

    #[test]
    fn case4() {
        // 需要 KMP 回退：第一次比到 [1,1,1] 失配後要退回 border
        let groups = vec![vec![1, 1, 2]];
        let nums = vec![1, 1, 1, 2];
        assert!(Solution::can_choose(groups, nums));
    }

    #[test]
    fn case5() {
        // group 比 nums 長
        let groups = vec![vec![1, 2, 3]];
        let nums = vec![1, 2];
        assert!(!Solution::can_choose(groups, nums));
    }

    #[test]
    fn case6() {
        // 同一個 group 出現多次，要依序各找一段不重疊的
        let groups = vec![vec![1, 2], vec![1, 2]];
        let nums = vec![1, 2, 1, 2];
        assert!(Solution::can_choose(groups, nums));
    }

    #[test]
    fn longest_border_table() {
        assert_eq!(
            Solution::build_long_border(&vec![1, 2, 1, 3, 1, 2, 1, 2]),
            vec![0, 0, 1, 0, 1, 2, 3, 2]
        );
    }
}
