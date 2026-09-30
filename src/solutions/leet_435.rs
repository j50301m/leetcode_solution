struct Solution {}

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by_key(|v| v[1]);

        let mut cnt = 0;
        let mut right = i32::MIN;
        for interval in intervals {
            if interval[0] < right {
                cnt += 1;
            } else {
                right = interval[1];
            }
        }
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(v: &[[i32; 2]]) -> i32 {
        Solution::erase_overlap_intervals(v.iter().map(|x| x.to_vec()).collect())
    }

    #[test]
    fn examples() {
        assert_eq!(run(&[[1, 2], [2, 3], [3, 4], [1, 3]]), 1);
        assert_eq!(run(&[[1, 2], [1, 2], [1, 2]]), 2);
        assert_eq!(run(&[[1, 2], [2, 3]]), 0);
    }

    #[test]
    fn edge_cases() {
        // 只有一個
        assert_eq!(run(&[[0, 1]]), 0);
        // 端點相接不算重疊
        assert_eq!(run(&[[1, 2], [2, 3], [3, 4], [4, 5]]), 0);
        // 一個大區間蓋住很多小的：刪大的那一個就好
        assert_eq!(run(&[[1, 100], [1, 2], [2, 3], [3, 4]]), 1);
        // 全部互相重疊，只能留一個
        assert_eq!(run(&[[1, 10], [2, 9], [3, 8], [4, 7]]), 3);
        // 負數
        assert_eq!(run(&[[-5, -1], [-3, 2], [0, 4]]), 1);
        // 鏈狀重疊：a-b 重疊、b-c 重疊，但 a-c 不重疊 → 刪中間 1 個
        assert_eq!(run(&[[1, 3], [2, 4], [3, 5]]), 1);
    }
}
