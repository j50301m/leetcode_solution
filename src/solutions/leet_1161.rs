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
use std::cmp::Reverse;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut map = HashMap::new();
        queue.push_front((root, 1));

        while let Some((node, layer)) = queue.pop_back() {
            let Some(node) = node else {
                continue;
            };

            let (val, left, right) = {
                let n = node.borrow();
                (n.val, n.left.clone(), n.right.clone())
            };

            *map.entry(layer).or_insert(0) += val;

            queue.push_front((left, layer + 1));
            queue.push_front((right, layer + 1));
        }

        map.into_iter()
            .max_by_key(|&(k, v)| (v, Reverse(k)))
            .map_or(0, |(k, _)| k)
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

    // Example 1：[1,7,0,7,-8]
    // 層總和 L1=1, L2=7, L3=-1 -> 最大是 L2 -> 回傳 2
    #[test]
    fn case1_example1() {
        let n7 = node(7, leaf(7), leaf(-8));
        let root = node(1, n7, leaf(0));
        assert_eq!(Solution::max_level_sum(root), 2);
    }

    // Example 2：[989,null,10250,98693,-89388,null,null,null,-32127]
    // L1=989, L2=10250, L3=9305, L4=-32127 -> 回傳 2
    #[test]
    fn case2_example2() {
        let n89388 = node(-89388, None, leaf(-32127));
        let n10250 = node(10250, leaf(98693), n89388);
        let root = node(989, None, n10250);
        assert_eq!(Solution::max_level_sum(root), 2);
    }

    // 單一節點 -> 只有第 1 層
    #[test]
    fn case3_single_node() {
        assert_eq!(Solution::max_level_sum(leaf(5)), 1);
    }

    // 全負數：L1=-1, L2=-5 -> 最大是 -1 在 L1
    // 抓「用 0 當初始值 / 挑到最小」的錯
    #[test]
    fn case4_all_negative() {
        let root = node(-1, leaf(-2), leaf(-3));
        assert_eq!(Solution::max_level_sum(root), 1);
    }

    // 平手：L1=1, L2=0+1=1 -> 題目要求回傳最小的層數 -> 1
    #[test]
    fn case5_tie_returns_smallest_level() {
        let root = node(1, leaf(0), leaf(1));
        assert_eq!(Solution::max_level_sum(root), 1);
    }

    // 最深層最大：L1=1, L2=5, L3=22 -> 3
    #[test]
    fn case6_deepest_level_wins() {
        let left = node(2, leaf(4), leaf(5));
        let right = node(3, leaf(6), leaf(7));
        let root = node(1, left, right);
        assert_eq!(Solution::max_level_sum(root), 3);
    }

    // 左斜樹 1-2-3-4 -> L4=4 最大 -> 4
    #[test]
    fn case7_left_skewed() {
        let root = node(1, node(2, node(3, leaf(4), None), None), None);
        assert_eq!(Solution::max_level_sum(root), 4);
    }
}
