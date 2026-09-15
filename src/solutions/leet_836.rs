struct Solution {}

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        rec1[0] < rec2[2] && rec2[0] < rec1[2] && rec1[1] < rec2[3] && rec2[1] < rec1[3]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 2, 2],
            vec![1, 1, 3, 3]
        ))
    }

    // 只共用一條邊，重疊面積為 0
    #[test]
    fn case2() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![1, 0, 2, 1]
        ))
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![2, 2, 3, 3]
        ))
    }

    // 只碰到一個角
    #[test]
    fn corner_touch() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![1, 1, 2, 2]
        ))
    }

    // rec2 完全被 rec1 包住，沒有任何頂點落在對方「嚴格內部」以外
    #[test]
    fn contained() {
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 10, 10],
            vec![2, 2, 3, 3]
        ))
    }

    #[test]
    fn contained_swapped() {
        assert!(Solution::is_rectangle_overlap(
            vec![2, 2, 3, 3],
            vec![0, 0, 10, 10]
        ))
    }

    // 十字交叉：橫長方形 × 直長方形，雙方都沒有頂點在對方內部
    #[test]
    fn cross_shape() {
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 3, 1],
            vec![1, -1, 2, 2]
        ))
    }

    #[test]
    fn cross_shape_swapped() {
        assert!(Solution::is_rectangle_overlap(
            vec![1, -1, 2, 2],
            vec![0, 0, 3, 1]
        ))
    }

    // 同一條水平帶，x 有交集但 rec2 上下超出 rec1
    #[test]
    fn same_x_band() {
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 5, 5],
            vec![1, -3, 2, 9]
        ))
    }

    #[test]
    fn identical() {
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 2, 2],
            vec![0, 0, 2, 2]
        ))
    }

    #[test]
    fn negative_coords() {
        assert!(Solution::is_rectangle_overlap(
            vec![-5, -5, -1, -1],
            vec![-3, -3, 4, 4]
        ))
    }
}
