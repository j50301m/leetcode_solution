struct Solution {}
impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let x = Ord::clamp(x_center, x1, x2);
        let y = Ord::clamp(y_center, y1, y2);

        let dx = x - x_center;
        let dy = y - y_center;

        if dx * dx + dy * dy <= radius * radius {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(r: i32, cx: i32, cy: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        Solution::check_overlap(r, cx, cy, x1, y1, x2, y2)
    }

    #[test]
    fn leetcode_examples() {
        assert!(run(1, 0, 0, 1, -1, 3, 1));
        assert!(!run(1, 1, 1, 1, -3, 2, -1));
        assert!(run(1, 0, 0, -1, 0, 0, 1));
    }

    #[test]
    fn center_inside_rectangle() {
        // 兩軸都沒被 clamp，距離 0，再小的半徑都算重疊
        assert!(run(1, 3, 3, 1, 1, 6, 4));
        assert!(run(1, 1, 1, 1, 1, 6, 4)); // 圓心壓在角上
    }

    #[test]
    fn edge_region_only_one_axis_clamped() {
        // 圓心在矩形正上方：x 沒被夾、y 被夾成 y2=4，最近點是垂足 (3,4)
        assert!(run(2, 3, 5, 1, 1, 6, 4)); // 距離 1 <= 2
        assert!(!run(1, 3, 6, 1, 1, 6, 4)); // 距離 2 > 1
                                            // 正左方：y 沒被夾，x 被夾成 x1=1
        assert!(run(2, -1, 3, 1, 1, 6, 4)); // 距離 2 <= 2，相切也算
        assert!(!run(1, -1, 3, 1, 1, 6, 4));
    }

    #[test]
    fn corner_region_both_axes_clamped() {
        // 圓心在右上角外側，最近點是角 (6,4)，距離 = 5 (3-4-5 直角三角形)
        assert!(!run(4, 9, 8, 1, 1, 6, 4));
        assert!(run(5, 9, 8, 1, 1, 6, 4)); // 相切
        assert!(run(6, 9, 8, 1, 1, 6, 4));
        // 左下角外側，同樣距離 5
        assert!(!run(4, -2, -2, 1, 1, 6, 4));
        assert!(run(5, -2, -2, 1, 1, 6, 4));
    }

    #[test]
    fn corner_gap_is_not_a_false_positive() {
        // 只看單軸會誤判：x 差 3、y 差 3 都 <= 4，但真正距離是 sqrt(18) ≈ 4.24 > 4
        assert!(!run(4, 9, 7, 1, 1, 6, 4));
    }

    #[test]
    fn degenerate_and_extremes() {
        // 退化成一條線段的矩形
        assert!(run(1, 0, 0, 0, -1, 0, 1));
        assert!(!run(1, 2, 0, 0, -1, 0, 1));
        // 題目上界：座標 ±10^4、半徑 2000，確認沒有 i32 溢位
        assert!(!run(2000, 10000, 10000, -10000, -10000, -9999, -9999));
        assert!(run(2000, -10000, -10000, -10000, -10000, 10000, 10000));
    }
}
