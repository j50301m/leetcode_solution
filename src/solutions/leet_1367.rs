struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut needs = Vec::new();
        let mut head = head;
        while let Some(node) = head {
            needs.push(node.val);
            head = node.next;
        }

        let long_borders = Self::build_longest_borders(&needs);
        Self::dfs(0, root, &needs, &long_borders)
    }

    fn dfs(
        mut border: usize,
        root: Option<Rc<RefCell<TreeNode>>>,
        needs: &Vec<i32>,
        longest_borders: &Vec<usize>,
    ) -> bool {
        if root.is_none() {
            return false;
        }

        let root = root.unwrap();
        let (val, left, right) = {
            let n = root.borrow();
            (n.val, n.left.clone(), n.right.clone())
        };

        while border > 0 && val != needs[border] {
            border = longest_borders[border - 1];
        }

        if needs[border] == val {
            border += 1;
        }

        if border == needs.len() {
            return true;
        }

        return Self::dfs(border, left, needs, longest_borders)
            || Self::dfs(border, right, needs, longest_borders);
    }

    fn build_longest_borders(nums: &Vec<i32>) -> Vec<usize> {
        let mut longest_border = vec![0; nums.len()];
        let mut border = 0;
        for i in 1..nums.len() {
            while border > 0 && nums[i] != nums[border] {
                border = longest_border[border - 1];
            }

            if nums[border] == nums[i] {
                border += 1;
            }

            longest_border[i] = border;
        }
        longest_border
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

    // 只往左長的一條直線，方便測「一條路徑上」的比對
    fn chain(values: &[i32]) -> Link {
        values
            .iter()
            .rev()
            .fold(None, |child, &val| node(val, child, None))
    }

    fn list(values: &[i32]) -> Option<Box<ListNode>> {
        values
            .iter()
            .rev()
            .fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
    }

    // 官方 example 的樹：
    //         1
    //       /   \
    //      4     4
    //       \   /
    //        2 2
    //       / / \
    //      1 6   8
    //           / \
    //          1   3
    fn example_tree() -> Link {
        let eight = node(8, leaf(1), leaf(3));
        let right_two = node(2, leaf(6), eight);
        let right_four = node(4, right_two, None);
        let left_two = node(2, leaf(1), None);
        let left_four = node(4, None, left_two);
        node(1, left_four, right_four)
    }

    // 官方 example 1：4 -> 2 -> 8 在右半邊，不是從 root 開始
    #[test]
    fn case1() {
        assert!(Solution::is_sub_path(list(&[4, 2, 8]), example_tree()));
    }

    // 官方 example 2：1 -> 4 -> 2 -> 6 從 root 開始
    #[test]
    fn case2() {
        assert!(Solution::is_sub_path(list(&[1, 4, 2, 6]), example_tree()));
    }

    // 官方 example 3：多一個 8，6 是葉子接不下去 -> false
    #[test]
    fn case3() {
        assert!(!Solution::is_sub_path(
            list(&[1, 4, 2, 6, 8]),
            example_tree()
        ));
    }

    // 單一節點：相同 / 不同
    #[test]
    fn case4() {
        assert!(Solution::is_sub_path(list(&[7]), leaf(7)));
        assert!(!Solution::is_sub_path(list(&[7]), leaf(8)));
    }

    // list 比整棵樹還長 -> false，抓 needs[border] 越界
    #[test]
    fn case5() {
        assert!(!Solution::is_sub_path(list(&[1, 2, 3]), chain(&[1, 2])));
    }

    // 比對要在中途重新開始：1 1 1 2 裡找 1 1 2
    // 抓「失配就把 border 歸 0」，正確應該退回 border = 1
    #[test]
    fn case6() {
        assert!(Solution::is_sub_path(
            list(&[1, 1, 2]),
            chain(&[1, 1, 1, 2])
        ));
    }

    // 失配時要「一直退」直到能接上或退到 0
    // 路徑 1 1 3 1 2 裡沒有 1 1 2；
    // 如果 fallback 用 if 只退一次，走到 3 時 border 會卡在 1（以為 "1" 有對上），
    // 接著 1 2 就湊出假的 1 1 2 -> 誤判 true
    #[test]
    fn case7() {
        assert!(!Solution::is_sub_path(
            list(&[1, 1, 2]),
            chain(&[1, 1, 3, 1, 2])
        ));
    }

    // 兄弟節點不算同一條路徑：1 的左 2、右 3，list 2 -> 3 -> false
    // 抓左子樹的比對狀態污染到右子樹
    #[test]
    fn case8() {
        let root = node(1, leaf(2), leaf(3));
        assert!(!Solution::is_sub_path(list(&[2, 3]), root.clone()));
        assert!(Solution::is_sub_path(list(&[1, 3]), root));
    }

    // 路徑必須連續：1 -> 2 -> 4 -> 3 裡沒有 1 2 3
    #[test]
    fn case9() {
        assert!(!Solution::is_sub_path(
            list(&[1, 2, 3]),
            chain(&[1, 2, 4, 3])
        ));
    }

    // 只在右子樹深處出現，左邊有一條差一點的誘餌
    #[test]
    fn case10() {
        let decoy = chain(&[5, 6, 9]);
        let target = node(0, None, node(5, None, node(6, None, leaf(7))));
        let root = node(0, decoy, target);
        assert!(Solution::is_sub_path(list(&[5, 6, 7]), root));
    }
}
