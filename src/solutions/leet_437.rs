// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut map: HashMap<i64, i32> = HashMap::new();
        map.insert(0, 1);
        Self::dfs(root, 0, target_sum as i64, &mut map)
    }

    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        parent_sum: i64,
        target: i64,
        map: &mut HashMap<i64, i32>,
    ) -> i32 {
        let Some(rc) = node else {
            return 0;
        };

        let (val, left, right) = {
            let n = rc.borrow();
            (n.val, n.left.clone(), n.right.clone())
        };

        let curr_sum = parent_sum + val as i64;
        let mut accumulation = 0;
        if let Some(cnt) = map.get(&(curr_sum - target)) {
            accumulation += cnt;
        };

        // Insert curr sum in to map
        *map.entry(curr_sum).or_insert(0) += 1;

        // Find the right leaf and left leaf
        accumulation += Self::dfs(right, curr_sum, target, map);
        accumulation += Self::dfs(left, curr_sum, target, map);

        map.entry(curr_sum).and_modify(|val| *val -= 1);

        accumulation
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type Link = Option<Rc<RefCell<TreeNode>>>;

    fn node(val: i32, left: Link, right: Link) -> Link {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    fn leaf(val: i32) -> Link {
        node(val, None, None)
    }

    // 空樹
    #[test]
    fn case1() {
        assert_eq!(Solution::path_sum(None, 8), 0);
    }

    // 只有 root，且自己就等於 target
    #[test]
    fn case2() {
        assert_eq!(Solution::path_sum(leaf(8), 8), 1);
        assert_eq!(Solution::path_sum(leaf(7), 8), 0);
    }

    // 官方 example 1：[10,5,-3,3,2,null,11,3,-2,null,1], target = 8
    // 命中 5->3、5->2->1、-3->11 三條
    #[test]
    fn case3() {
        let root = node(
            10,
            node(5, node(3, leaf(3), leaf(-2)), node(2, None, leaf(1))),
            node(-3, None, leaf(11)),
        );
        assert_eq!(Solution::path_sum(root, 8), 3);
    }

    // 官方 example 2：[5,4,8,11,null,13,4,7,2,null,null,5,1], target = 22
    // 命中 5->4->11->2、5->8->4->5、4->11->7 三條
    #[test]
    fn case4() {
        let root = node(
            5,
            node(4, node(11, leaf(7), leaf(2)), None),
            node(8, leaf(13), node(4, leaf(5), leaf(1))),
        );
        assert_eq!(Solution::path_sum(root, 22), 3);
    }

    // 路徑從 root 起算 (1->2 = 3) -> 抓 map 忘記放 {0: 1}，少了空前綴就會回 0
    #[test]
    fn case5() {
        assert_eq!(Solution::path_sum(node(1, leaf(2), None), 3), 1);
    }

    // 只有節點 2 自己等於 target -> 抓忘記回溯 map[cur] -= 1
    // 少了回溯的話，走到右子 4 時會查到左子 2 留下的前綴和而多算一筆 (回 2)
    #[test]
    fn case6() {
        assert_eq!(Solution::path_sum(node(1, leaf(2), leaf(4)), 2), 1);
    }

    // 同一條路徑上出現兩次相同前綴和 (10 -> 5 -> -5 -> 3，前綴和 10 出現兩次)
    // 命中 [3] 和 [5,-5,3] 兩條 -> 抓 map 的 value 用 bool/HashSet 而不是計數
    #[test]
    fn case7() {
        let root = node(10, node(5, node(-5, leaf(3), None), None), None);
        assert_eq!(Solution::path_sum(root, 3), 2);
    }

    // target = 0 且節點值都是 0：[0](root)、[0](child)、[0,0] 共 3 條
    // -> 抓「先查表再把自己放進 map」的順序，放反了會把自己算成自己的起點
    #[test]
    fn case8() {
        assert_eq!(Solution::path_sum(node(0, leaf(0), None), 0), 3);
    }

    // 全負數，target 也是負的 (-2 -> -3 = -5)
    #[test]
    fn case9() {
        assert_eq!(Solution::path_sum(node(-2, leaf(-3), None), -5), 1);
    }

    // 三個 1e9 串成一條鏈，前綴和到 3e9 -> 抓前綴和用 i32 造成的溢位
    #[test]
    fn case10() {
        let big = 1_000_000_000;
        let root = node(big, node(big, leaf(big), None), None);
        assert_eq!(Solution::path_sum(root, big), 3);
    }
}
