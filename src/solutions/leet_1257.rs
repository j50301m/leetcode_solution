use std::collections::{HashMap, HashSet};
use std::iter::successors;

struct Solution;

impl Solution {
    pub fn find_smallest_region(
        regions: Vec<Vec<String>>,
        region1: String,
        region2: String,
    ) -> String {
        // child -> parent；每列第 0 個是父，其餘都是子
        let parent: HashMap<&str, &str> = regions
            .iter()
            .flat_map(|row| row[1..].iter().map(move |c| (c.as_str(), row[0].as_str())))
            .collect();

        // successors：從自己出發反覆套 parent.get，直到 None（root）為止，路徑含自己
        let ancestors: HashSet<&str> =
            successors(Some(region1.as_str()), |n| parent.get(n).copied()).collect();

        successors(Some(region2.as_str()), |n| parent.get(n).copied())
            .find(|n| ancestors.contains(n))
            .unwrap_or_default()
            .to_string()
    }
}

// 原始版本（保留備查）：
//
// impl Solution {
//     pub fn find_smallest_region(
//         regions: Vec<Vec<String>>,
//         region1: String,
//         region2: String,
//     ) -> String {
//         let mut map: HashMap<&str, &str> = HashMap::new();
//         // Build a map
//         for region_list in regions.iter() {
//             for (i, region) in region_list.iter().enumerate() {
//                 if i == 0 {
//                     continue;
//                 }
//                 map.insert(region, &region_list[0]);
//             }
//         }
//
//         // Build a regin1 parent's set
//         let mut set: HashSet<&str> = HashSet::new();
//         let mut curr = region1.as_str();
//         set.insert(curr);
//         while let Some(&parent) = map.get(curr) {
//             set.insert(parent);
//             curr = parent;
//         }
//
//         // Find region2 itself
//         let mut curr = region2.as_str();
//         if let Some(&parent) = set.get(curr) {
//             return parent.to_string();
//         }
//
//         // Find regin2 parents
//         while let Some(&parent) = map.get(curr) {
//             if let Some(&found) = set.get(parent) {
//                 return found.to_string();
//             }
//             curr = parent;
//         }
//
//         String::new()
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    // LeetCode 範例的那棵樹
    const EARTH: &[&[&str]] = &[
        &["Earth", "North America", "South America"],
        &["North America", "United States", "Canada"],
        &["United States", "New York", "Boston"],
        &["Canada", "Ontario", "Quebec"],
        &["South America", "Brazil"],
    ];

    fn call(rows: &[&[&str]], a: &str, b: &str) -> String {
        let regions = rows
            .iter()
            .map(|row| row.iter().map(|s| s.to_string()).collect())
            .collect();
        Solution::find_smallest_region(regions, a.to_string(), b.to_string())
    }

    #[test]
    fn leetcode_example() {
        assert_eq!(call(EARTH, "Quebec", "New York"), "North America");
    }

    #[test]
    fn cases() {
        let cases: &[(&str, &str, &str)] = &[
            ("Quebec", "New York", "North America"), // 分屬兩棵子樹，往上才會合流
            ("New York", "Quebec", "North America"), // 交換順序答案不變
            ("Canada", "Quebec", "Canada"),          // region1 是 region2 的祖先
            ("Quebec", "Canada", "Canada"), // region2 是 region1 的祖先（走進迴圈前的那個判斷）
            ("Canada", "Canada", "Canada"), // 同一個節點，自己就是自己的祖先
            ("Ontario", "Quebec", "Canada"), // 同一個父節點
            ("Brazil", "Boston", "Earth"),  // 只有 root 共通
            ("Earth", "Boston", "Earth"),   // region1 就是 root
            ("Boston", "Earth", "Earth"),   // region2 就是 root，region1 要一路爬到頂
        ];
        for (r1, r2, want) in cases {
            assert_eq!(call(EARTH, r1, r2), *want, "({}, {})", r1, r2);
        }
    }

    // regions 沒有保證父列會排在子列前面，建表不該依賴輸入順序
    #[test]
    fn row_order_does_not_matter() {
        let shuffled: &[&[&str]] = &[
            &["Canada", "Ontario", "Quebec"],
            &["South America", "Brazil"],
            &["United States", "New York", "Boston"],
            &["Earth", "North America", "South America"],
            &["North America", "United States", "Canada"],
        ];
        assert_eq!(call(shuffled, "Quebec", "New York"), "North America");
        assert_eq!(call(shuffled, "Brazil", "Boston"), "Earth");
    }

    // 兩條長鏈掛在同一個 root：兩次往上都得走滿全長才會相遇
    #[test]
    fn deep_chains() {
        const N: usize = 500;
        let mut rows: Vec<Vec<String>> =
            vec![vec!["r".to_string(), "a0".to_string(), "b0".to_string()]];
        for i in 0..N - 1 {
            rows.push(vec![format!("a{}", i), format!("a{}", i + 1)]);
            rows.push(vec![format!("b{}", i), format!("b{}", i + 1)]);
        }

        let last_a = format!("a{}", N - 1);
        let last_b = format!("b{}", N - 1);
        assert_eq!(
            Solution::find_smallest_region(rows.clone(), last_a.clone(), last_b),
            "r"
        );
        // 同一條鏈上，答案是比較淺的那個
        assert_eq!(
            Solution::find_smallest_region(rows, last_a, "a250".to_string()),
            "a250"
        );
    }
}
