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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::get_leafs(root1) == Self::get_leafs(root2)
    }

    fn get_leafs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        if let Some(node) = root {
            stack.push(node);
        }

        let mut ans: Vec<i32> = Vec::new();
        while let Some(curr) = stack.pop() {
            let n = curr.borrow();

            if n.left.is_none() && n.right.is_none() {
                ans.push(n.val);
                continue;
            }

            // 先 push right 再 push left：stack 後進先出，left 才會先被拿出來
            // 這樣收到的葉子順序才是題目要的「從左到右」
            if let Some(right) = &n.right {
                stack.push(right.clone());
            }
            if let Some(left) = &n.left {
                stack.push(left.clone());
            }
        }
        ans
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

    // 官方 example 1：兩棵形狀不同、葉子序列都是 [6,7,4,9,8]
    // 葉子深度不一致 -> 抓用 BFS 分層收集（BFS 會收成 [6,9,8,7,4]）
    #[test]
    fn case1() {
        let t1 = node(
            3,
            node(5, leaf(6), node(2, leaf(7), leaf(4))),
            node(1, leaf(9), leaf(8)),
        );
        let t2 = node(
            3,
            node(5, leaf(6), leaf(7)),
            node(1, leaf(4), node(2, leaf(9), leaf(8))),
        );
        assert!(Solution::leaf_similar(t1, t2));
    }

    // 官方 example 2：葉子集合相同但順序相反 -> 抓「用 HashSet / 排序後比較」
    #[test]
    fn case2() {
        assert!(!Solution::leaf_similar(
            node(1, leaf(2), leaf(3)),
            node(1, leaf(3), leaf(2))
        ));
    }

    // 最小的深淺反例：A 的 3 比 5 深，但從左到右仍然是 3 先
    // -> 這是 BFS 版最短的紅燈
    #[test]
    fn case3() {
        let a = node(1, node(2, leaf(3), None), leaf(5));
        let b = node(1, leaf(3), leaf(5));
        assert!(Solution::leaf_similar(a, b));
    }

    // 上一筆的鏡像：右邊比較深 -> 抓 push 順序反了（先 left 再 right 會變從右到左）
    #[test]
    fn case4() {
        let a = node(1, leaf(3), node(2, None, leaf(5)));
        let b = node(1, leaf(3), leaf(5));
        assert!(Solution::leaf_similar(a, b));
    }

    // 只有 root，root 自己就是葉子
    #[test]
    fn case5() {
        assert!(Solution::leaf_similar(leaf(1), leaf(1)));
    }

    // 兩棵單節點但值不同
    #[test]
    fn case6() {
        assert!(!Solution::leaf_similar(leaf(1), leaf(2)));
    }

    // 葉子數量不同（前綴相同）-> 抓「只比到較短的那一段就回 true」
    #[test]
    fn case7() {
        let a = node(1, leaf(3), node(2, leaf(5), leaf(7)));
        let b = node(1, leaf(3), leaf(5));
        assert!(!Solution::leaf_similar(a, b));
    }

    // 形狀天差地遠但葉子序列都是 [7,8]
    #[test]
    fn case8() {
        let a = node(1, node(2, node(3, leaf(7), None), None), leaf(8));
        let b = node(1, leaf(7), node(9, None, leaf(8)));
        assert!(Solution::leaf_similar(a, b));
    }

    // 只有一邊有子樹的斜樹，葉子只有一個
    #[test]
    fn case9() {
        let a = node(1, node(2, node(3, leaf(4), None), None), None);
        assert!(Solution::leaf_similar(a, leaf(4)));
    }
}
