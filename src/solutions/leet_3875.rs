struct Solution {}

impl Solution {
    pub fn uniform_array(_nums1: Vec<i32>) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 官方 example 1：[2,3] 混合 -> 2-3=-1，湊成全奇
    #[test]
    fn case1() {
        assert!(Solution::uniform_array(vec![2, 3]));
    }

    // 官方 example 2：[4,6] 全偶 -> 原封不動
    #[test]
    fn case2() {
        assert!(Solution::uniform_array(vec![4, 6]));
    }

    // n == 1：沒有 j != i 可選，只能原封不動，單一元素必然同奇偶
    #[test]
    fn case3() {
        assert!(Solution::uniform_array(vec![7]));
        assert!(Solution::uniform_array(vec![8]));
    }

    // O == 0（全偶）：唯一走「全偶」那條路的情況
    #[test]
    fn case4() {
        assert!(Solution::uniform_array(vec![2, 4, 6, 8, 100]));
    }

    // O == n（全奇）：留著不動就是全奇
    #[test]
    fn case5() {
        assert!(Solution::uniform_array(vec![1, 3, 5, 7, 99]));
    }

    // O == 1：唯一那個奇數翻不了自己，「全偶」這條路走不通
    // -> 抓「只推目標全偶就回 false」的錯誤解法
    #[test]
    fn case6() {
        assert!(Solution::uniform_array(vec![2, 4, 3]));
        assert!(Solution::uniform_array(vec![3, 2]));
    }

    // O == 1 且奇數在最後 / 偶數只有一個 -> 同上，換個排列位置
    #[test]
    fn case7() {
        assert!(Solution::uniform_array(vec![1, 3, 5, 2]));
    }

    // n = 100 的滿載輸入，奇偶各半
    #[test]
    fn case8() {
        assert!(Solution::uniform_array((1..=100).collect()));
    }
}
