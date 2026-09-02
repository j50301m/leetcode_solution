struct Solution {}

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
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack: Vec<(Rc<RefCell<TreeNode>>, i32)> = Vec::new();

        if let Some(node) = root {
            stack.push((node.clone(), node.borrow().val));
        }

        let mut total = 0;
        while !stack.is_empty() {
            let (node, max) = stack.pop().unwrap();
            let n = node.borrow();
            let mut max = max;
            if n.val >= max {
                total += 1;
                max = n.val;
            }

            if let Some(right) = &n.right {
                stack.push((right.clone(), max));
            }
            if let Some(left) = &n.left {
                stack.push((left.clone(), max));
            }
        }

        total
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

    // 官方 example 1：[3,1,4,3,null,1,5]
    #[test]
    fn case1() {
        assert_eq!(
            Solution::good_nodes(node(
                3,
                node(1, leaf(3), None),
                node(4, leaf(1), leaf(5))
            )),
            4
        );
    }

    // 官方 example 2：[3,3,null,4,2]
    // 路徑上有同值的 3 仍算 good -> 抓判斷寫成 n.val > max
    #[test]
    fn case2() {
        assert_eq!(
            Solution::good_nodes(node(3, node(3, leaf(4), leaf(2)), None)),
            3
        );
    }

    // 官方 example 3：只有 root
    #[test]
    fn case3() {
        assert_eq!(Solution::good_nodes(leaf(1)), 1);
    }

    // 一路遞減，只有 root 是 good
    #[test]
    fn case4() {
        assert_eq!(Solution::good_nodes(node(5, node(4, leaf(3), None), leaf(2))), 1);
    }

    // 全部同值 -> 每個都是 good，再抓一次 >= 而不是 >
    #[test]
    fn case5() {
        assert_eq!(Solution::good_nodes(node(2, node(2, leaf(2), None), leaf(2))), 4);
    }

    // 全負數 -> 抓 path_max 初值寫成 0
    #[test]
    fn case6() {
        assert_eq!(Solution::good_nodes(node(-5, leaf(-6), leaf(-4))), 2);
    }

    // 1 -> 5 -> 3：3 的路徑最大值是 5，不是 root 的 1
    // -> 抓小孩沿用「還沒更新過的 max」（那樣 3 會被誤算成 good，變 3）
    #[test]
    fn case7() {
        assert_eq!(
            Solution::good_nodes(node(1, None, node(5, None, leaf(3)))),
            2
        );
    }

    // 空樹
    #[test]
    fn case8() {
        assert_eq!(Solution::good_nodes(None), 0);
    }
}
