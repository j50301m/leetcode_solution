use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let mut pos1 = Vec::new();
        let mut pos2 = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if img1[i][j] == 1 {
                    pos1.push((i as i32, j as i32));
                }
                if img2[i][j] == 1 {
                    pos2.push((i as i32, j as i32));
                }
            }
        }

        let mut map = HashMap::new();
        for &(r1, c1) in pos1.iter() {
            for &(r2, c2) in pos2.iter() {
                let delta_r = r1 - r2;
                let delta_c = c1 - c2;
                *map.entry((delta_r, delta_c)).or_insert(0) += 1;
            }
        }

        map.iter().map(|(_k, v)| *v).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
            ),
            3
        )
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::largest_overlap(vec![vec![1]], vec![vec![1]]), 1)
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::largest_overlap(vec![vec![0]], vec![vec![0]]), 0)
    }

    #[test]
    fn all_zero() {
        assert_eq!(
            Solution::largest_overlap(vec![vec![0, 0], vec![0, 0]], vec![vec![0, 0], vec![0, 0]]),
            0
        )
    }

    #[test]
    fn one_side_empty() {
        assert_eq!(
            Solution::largest_overlap(vec![vec![1, 1], vec![1, 1]], vec![vec![0, 0], vec![0, 0]]),
            0
        )
    }

    #[test]
    fn identical() {
        assert_eq!(
            Solution::largest_overlap(vec![vec![1, 1], vec![1, 1]], vec![vec![1, 1], vec![1, 1]]),
            4
        )
    }

    // 需要往左上移動，檢查負位移
    #[test]
    fn negative_shift() {
        assert_eq!(
            Solution::largest_overlap(vec![vec![0, 0], vec![0, 1]], vec![vec![1, 0], vec![0, 0]]),
            1
        )
    }

    // 對角線兩個 1：最佳解是不位移拿 2，而不是位移後只拿 1
    #[test]
    fn diagonal_prefers_no_shift() {
        assert_eq!(
            Solution::largest_overlap(
                vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]],
                vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]
            ),
            2
        )
    }
}
