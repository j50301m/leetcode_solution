struct Solution {}

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let min = nums1.iter().min().unwrap();

        if min % 2 == 0 {
            for i in 0..nums1.len() {
                if nums1[i] % 2 != 0 {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 官方 example 1：[1,4,7] -> 4-1=3，湊成全奇
    #[test]
    fn case1() {
        assert!(Solution::uniform_array(vec![1, 4, 7]));
    }

    // 官方 example 2：[2,3] -> 2 找不到比自己小的奇數；3 也找不到比自己小的奇數
    #[test]
    fn case2() {
        assert!(!Solution::uniform_array(vec![2, 3]));
    }

    // 官方 example 3：[4,6] 全偶 -> 原封不動
    #[test]
    fn case3() {
        assert!(Solution::uniform_array(vec![4, 6]));
    }

    // n == 1：沒有 j != i 可選，只能原封不動，單一元素必然同奇偶
    #[test]
    fn case4() {
        assert!(Solution::uniform_array(vec![7]));
        assert!(Solution::uniform_array(vec![8]));
    }

    // 全偶：唯一能走「全偶」那條路的情況
    #[test]
    fn case5() {
        assert!(Solution::uniform_array(vec![2, 4, 6, 8, 100]));
    }

    // 全奇：留著不動就是全奇
    #[test]
    fn case6() {
        assert!(Solution::uniform_array(vec![1, 3, 5, 7, 99]));
    }

    // 最小值是奇數，但不在 index 0 -> 抓「只看 nums1[0] 的奇偶」
    // 1 比所有偶數小，4-1=3、6-1=5，全奇可行
    #[test]
    fn case7() {
        assert!(Solution::uniform_array(vec![4, 1, 6]));
        assert!(Solution::uniform_array(vec![2, 1]));
    }

    // nums1[0] 是奇數但不是最小值 -> 抓「開頭是奇數就直接 true」
    // 2 找不到比它小的奇數，全奇不可行；3 翻不成偶數，全偶也不可行
    #[test]
    fn case8() {
        assert!(!Solution::uniform_array(vec![3, 2]));
        assert!(!Solution::uniform_array(vec![3, 5, 2]));
    }

    // 只有一個偶數，且它比最小奇數大 -> 可行
    #[test]
    fn case9() {
        assert!(Solution::uniform_array(vec![1, 3, 5, 2]));
    }

    // 混合但最小的偶數卡在最小奇數下面 -> 不可行
    // 最小奇數是 5，偶數 4 找不到比自己小的奇數
    #[test]
    fn case10() {
        assert!(!Solution::uniform_array(vec![9, 5, 4, 100]));
    }

    // 值域上界 1e9，差值不會溢位；最小值 3 是奇數 -> 可行
    #[test]
    fn case11() {
        assert!(Solution::uniform_array(vec![1_000_000_000, 3, 999_999_999]));
    }

    // n = 1e5 滿載，1 是最小值且為奇數 -> 可行（順便看 O(n) 有沒有寫成 O(n^2)）
    #[test]
    fn case12() {
        assert!(Solution::uniform_array((1..=100_000).collect()));
    }
}
