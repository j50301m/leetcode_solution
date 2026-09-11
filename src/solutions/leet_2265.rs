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
use std::rc::Rc;
struct Solution {}

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut matched = 0;
        Self::dfs(root, &mut matched);
        matched
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, matched: &mut i32) -> (i32, i32) {
        let Some(node) = node else {
            return (0, 0);
        };

        let (val, left, right) = {
            let n = node.borrow();
            (n.val, n.left.clone(), n.right.clone())
        };

        let mut sum = val;
        let mut node_cnt = 1; // Self node
        let (left_sum, left_count) = Self::dfs(left, matched);
        let (right_sum, right_count) = Self::dfs(right, matched);

        sum += left_sum + right_sum;
        node_cnt += left_count + right_count;
        if sum / node_cnt == val {
            *matched += 1;
        }

        (sum, node_cnt)
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

    // Example 1：[4,8,5,0,1,null,6] -> 5
    // 命中的是 4、0、1、5、6；8 的子樹平均是 3 不算
    #[test]
    fn case1_example1() {
        let n8 = node(8, leaf(0), leaf(1));
        let n5 = node(5, None, leaf(6));
        let root = node(4, n8, n5);
        assert_eq!(Solution::average_of_subtree(root), 5);
    }

    // Example 2：[1] -> 1，單節點自己一定命中
    #[test]
    fn case2_single_node() {
        assert_eq!(Solution::average_of_subtree(leaf(1)), 1);
    }

    // 空樹 -> 0（抓 dfs 對 None 的處理，別除以 0）
    #[test]
    fn case3_empty() {
        assert_eq!(Solution::average_of_subtree(None), 0);
    }

    // 只有葉子命中：root=1, 子樹平均 (1+2+3)/3 = 2 != 1
    #[test]
    fn case4_only_leaves_match() {
        let root = node(1, leaf(2), leaf(3));
        assert_eq!(Solution::average_of_subtree(root), 2);
    }

    // 無條件捨去：root=1, sum=4, cnt=3 -> 4/3 = 1 命中
    // 抓「四捨五入」或「用浮點比較」的錯
    #[test]
    fn case5_floor_division() {
        let root = node(1, leaf(0), leaf(3));
        assert_eq!(Solution::average_of_subtree(root), 3);
    }

    // 左斜樹 1-2-3：node3 ✓、node2 (5/2=2) ✓、node1 (6/3=2) ✗ -> 2
    #[test]
    fn case6_left_skewed() {
        let root = node(1, node(2, leaf(3), None), None);
        assert_eq!(Solution::average_of_subtree(root), 2);
    }

    // 全部同值 -> 每個節點都命中
    #[test]
    fn case7_all_same_value() {
        let root = node(7, node(7, leaf(7), leaf(7)), leaf(7));
        assert_eq!(Solution::average_of_subtree(root), 5);
    }
}
