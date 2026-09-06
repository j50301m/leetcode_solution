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
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(node) = root else {
            return 0;
        };
        let (left_child, right_child) = {
            let n = node.borrow();
            (n.left.clone(), n.right.clone())
        };

        let left = Self::dfs(left_child, true, 0);
        let right = Self::dfs(right_child, false, 0);

        left.max(right)
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool, path: i32) -> i32 {
        let Some(node) = node else {
            return path;
        };

        let (left_child, right_child) = {
            let n = node.borrow();
            (n.left.clone(), n.right.clone())
        };

        let left_path = if is_left { 0 } else { path + 1 };
        let right_path = if !is_left { 0 } else { path + 1 };
        let candidate_1 = Self::dfs(left_child, true, left_path);
        let candidate_2 = Self::dfs(right_child, false, right_path);

        candidate_1.max(candidate_2)
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

    // 官方 example 1：答案 3（root 右 -> 右 -> 左 -> 右 這條不是從 root 起算）
    // 最長那條是 A -右-> C -左-> D -右-> F
    #[test]
    fn case1() {
        let f = node(1, None, leaf(1));
        let d = node(1, None, f);
        let c = node(1, d, leaf(1));
        let a = node(1, leaf(1), c);
        let root = node(1, None, a);
        assert_eq!(Solution::longest_zig_zag(root), 3);
    }

    // 官方 example 2：答案 4，整條從 root 開始：左 右 左 右
    #[test]
    fn case2() {
        let p = node(1, None, leaf(1));
        let m = node(1, p, leaf(1));
        let l = node(1, None, m);
        let root = node(1, l, leaf(1));
        assert_eq!(Solution::longest_zig_zag(root), 4);
    }

    // 官方 example 3：只有 root，長度以「邊」計 -> 0
    // 抓「回傳節點數而不是邊數」跟那個 ans < 3 就 -1 的分支
    #[test]
    fn case3() {
        assert_eq!(Solution::longest_zig_zag(leaf(1)), 0);
    }

    // 最小的一步：root 有兩個小孩，任一條邊都是合法 zigzag -> 1
    #[test]
    fn case4() {
        assert_eq!(Solution::longest_zig_zag(node(1, leaf(2), leaf(3))), 1);
    }

    // 右-右直線：同方向不算 zigzag，答案只有 1
    // 抓 root 呼叫右子樹時 is_left 傳錯（傳 true）導致方向判斷整棵右子樹都反了
    #[test]
    fn case5() {
        let root = node(1, None, node(2, None, leaf(3)));
        assert_eq!(Solution::longest_zig_zag(root), 1);
    }

    // 左-左直線：case5 的鏡像，答案一樣是 1
    #[test]
    fn case6() {
        let root = node(1, node(2, leaf(3), None), None);
        assert_eq!(Solution::longest_zig_zag(root), 1);
    }

    // 從 root 出發、右邊起手的完整 zigzag：右 左 右 -> 3
    #[test]
    fn case7() {
        let root = node(1, None, node(2, node(3, None, leaf(4)), None));
        assert_eq!(Solution::longest_zig_zag(root), 3);
    }

    // 中途要重設：root -左-> L1 -左-> L2 -右-> R -左-> X
    // root->L1->L2 連兩次左，計數要從 L1->L2 這條邊重新算，
    // 最長的是 L1 -左-> L2 -右-> R -左-> X = 3（不是 4）
    // 抓「同方向時沒有 reset，一路累加成 4」
    #[test]
    fn case8() {
        let l2 = node(3, None, node(4, leaf(5), None));
        let l1 = node(2, l2, None);
        let root = node(1, l1, None);
        assert_eq!(Solution::longest_zig_zag(root), 3);
    }

    // 全左斜樹（5 個節點）：答案還是 1
    #[test]
    fn case9() {
        let root = node(
            1,
            node(2, node(3, node(4, leaf(5), None), None), None),
            None,
        );
        assert_eq!(Solution::longest_zig_zag(root), 1);
    }

    // 左右兩邊都有 zigzag，右邊比較長 -> 取最大值
    // 抓「只回傳左子樹結果」或 max 取錯邊
    #[test]
    fn case10() {
        let left = node(2, None, leaf(4)); // 左 右 = 2
        let right = node(3, node(5, None, leaf(6)), None); // 右 左 右 = 3
        assert_eq!(Solution::longest_zig_zag(node(1, left, right)), 3);
    }
}
