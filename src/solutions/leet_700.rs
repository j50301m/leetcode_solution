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
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut curr = root;
        while let Some(node) = curr {
            let v = node.borrow().val;

            if v == val {
                return Some(node);
            }

            // Binary search Tree
            if val > v {
                curr = node.borrow().right.clone();
            } else {
                curr = node.borrow().left.clone();
            }
        }

        None
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

    // 官方範例的樹 [4,2,7,1,3]，回傳 (root, 節點2)
    fn example_tree() -> (Link, Link) {
        let n2 = node(2, leaf(1), leaf(3));
        let root = node(4, n2.clone(), leaf(7));
        (root, n2)
    }

    // Example 1：找 2 -> 回傳以 2 為根的子樹 [2,1,3]
    // 順便確認回傳的節點「子樹還在」，不是只有那顆節點
    #[test]
    fn case1_example1() {
        let (root, n2) = example_tree();
        assert_eq!(Solution::search_bst(root, 2), n2);
    }

    // Example 2：找 5 -> 樹裡沒有 -> None
    // 抓「跳出迴圈忘了回 None」
    #[test]
    fn case2_example2_not_found() {
        let (root, _) = example_tree();
        assert_eq!(Solution::search_bst(root, 5), None);
    }

    // 目標就是 root -> 第一步就命中，回傳整棵樹
    #[test]
    fn case3_target_is_root() {
        let (root, _) = example_tree();
        assert_eq!(Solution::search_bst(root.clone(), 4), root);
    }

    // 目標在最左下角 -> 連續往左走兩次
    #[test]
    fn case4_leftmost_leaf() {
        let (root, _) = example_tree();
        assert_eq!(Solution::search_bst(root, 1), leaf(1));
    }

    // 目標在右邊 -> val > v 那條分支
    // 抓「左右判斷寫反」
    #[test]
    fn case5_right_branch() {
        let (root, _) = example_tree();
        assert_eq!(Solution::search_bst(root, 7), leaf(7));
    }

    // 比全樹最小值還小 -> 一路往左走到 None
    #[test]
    fn case6_smaller_than_all() {
        let (root, _) = example_tree();
        assert_eq!(Solution::search_bst(root, 0), None);
    }

    // 比全樹最大值還大 -> 一路往右走到 None
    #[test]
    fn case7_larger_than_all() {
        let (root, _) = example_tree();
        assert_eq!(Solution::search_bst(root, 100), None);
    }

    // 單一節點：命中 / 沒命中
    #[test]
    fn case8_single_node() {
        assert_eq!(Solution::search_bst(leaf(4), 4), leaf(4));
        assert_eq!(Solution::search_bst(leaf(4), 2), None);
    }

    // 右斜 BST 1-2-3-4，找最深的 4 -> 走滿 h 步
    #[test]
    fn case9_right_skewed() {
        let n4 = leaf(4);
        let root = node(1, None, node(2, None, node(3, None, n4.clone())));
        assert_eq!(Solution::search_bst(root, 4), n4);
    }

    // 左斜 BST 4-3-2-1，找最深的 1
    #[test]
    fn case10_left_skewed() {
        let n1 = leaf(1);
        let root = node(4, node(3, node(2, n1.clone(), None), None), None);
        assert_eq!(Solution::search_bst(root, 1), n1);
    }
}
