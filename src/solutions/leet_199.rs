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
use std::collections::VecDeque;
use std::rc::Rc;
struct Solution {}

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        let mut curr_layer = 1;

        queue.push_front((root, 1));
        while let Some((node, layer)) = queue.pop_back() {
            let Some(node) = node else {
                continue;
            };

            let (val, left, right) = {
                let n = node.borrow();
                (n.val, n.left.clone(), n.right.clone())
            };

            if layer == curr_layer {
                result.push(val);
                curr_layer += 1;
            }

            queue.push_front((right, layer + 1));
            queue.push_front((left, layer + 1));
        }

        result
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

    // Example 1：[1,2,3,null,5,null,4] -> [1,3,4]
    #[test]
    fn case1_example1() {
        let n2 = node(2, None, leaf(5));
        let n3 = node(3, None, leaf(4));
        let root = node(1, n2, n3);
        assert_eq!(Solution::right_side_view(root), vec![1, 3, 4]);
    }

    // Example 2：[1,null,3] -> [1,3]
    #[test]
    fn case2_only_right() {
        let root = node(1, None, leaf(3));
        assert_eq!(Solution::right_side_view(root), vec![1, 3]);
    }

    // 空樹 -> []
    #[test]
    fn case3_empty() {
        assert_eq!(Solution::right_side_view(None), Vec::<i32>::new());
    }

    // 左斜樹 1-2-3：每層最右邊其實在左邊
    // 抓「只往 right 走」的偷懶解法
    #[test]
    fn case4_left_skewed() {
        let root = node(1, node(2, leaf(3), None), None);
        assert_eq!(Solution::right_side_view(root), vec![1, 2, 3]);
    }

    // 右子樹比左子樹淺：第 3 層只有左邊的 4
    // 抓「右邊沒了就停」以及 layer 記錯的 off-by-one
    #[test]
    fn case5_left_deeper_than_right() {
        let root = node(1, node(2, leaf(4), None), leaf(3));
        assert_eq!(Solution::right_side_view(root), vec![1, 3, 4]);
    }

    // 滿二元樹，確認每層都挑到最右
    #[test]
    fn case6_full_tree() {
        let left = node(2, leaf(4), leaf(5));
        let right = node(3, leaf(6), leaf(7));
        let root = node(1, left, right);
        assert_eq!(Solution::right_side_view(root), vec![1, 3, 7]);
    }

    // 單一節點
    #[test]
    fn case7_single_node() {
        assert_eq!(Solution::right_side_view(leaf(42)), vec![42]);
    }
}
