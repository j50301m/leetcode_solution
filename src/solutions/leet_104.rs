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

struct Solution {}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        if let Some(node) = root {
            queue.push_back(node);
        }

        let mut layer = 0;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let curr = queue.pop_front().unwrap();
                let node = curr.borrow();
                if let Some(left) = &node.left {
                    queue.push_back(left.clone());
                }
                if let Some(right) = &node.right {
                    queue.push_back(right.clone());
                }
            }
            layer += 1;
        }

        layer
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
        assert_eq!(Solution::max_depth(None), 0);
    }

    // 只有 root
    #[test]
    fn case2() {
        assert_eq!(Solution::max_depth(leaf(1)), 1);
    }

    // 官方 example 1：[3,9,20,null,null,15,7]
    // 5 個節點但深度只有 3 -> 抓「layer += 1 放在內層迴圈裡」變成數節點數
    #[test]
    fn case3() {
        assert_eq!(
            Solution::max_depth(node(3, leaf(9), node(20, leaf(15), leaf(7)))),
            3
        );
    }

    // 官方 example 2：[1,null,2]，只有右子樹
    #[test]
    fn case4() {
        assert_eq!(Solution::max_depth(node(1, None, leaf(2))), 2);
    }

    // 左邊細長 (1-2-3-4)、右邊只有一層 -> 抓只走右邊或取錯 max
    #[test]
    fn case5() {
        assert_eq!(
            Solution::max_depth(node(1, node(2, node(3, leaf(4), None), None), leaf(5))),
            4
        );
    }

    // 上一筆的鏡像：深度在右邊 -> 抓只走左邊
    #[test]
    fn case6() {
        assert_eq!(
            Solution::max_depth(node(1, leaf(2), node(3, None, node(4, None, leaf(5))))),
            4
        );
    }

    // 滿二元樹：7 個節點，深度 3 -> 數節點數的話會回 7，差距最明顯
    #[test]
    fn case7() {
        assert_eq!(
            Solution::max_depth(node(
                1,
                node(2, leaf(4), leaf(5)),
                node(3, leaf(6), leaf(7))
            )),
            3
        );
    }
}
