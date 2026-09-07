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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p?.borrow().val;
        let q = q?.borrow().val;
        Self::dfs(root, p, q)
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let node = node?;

        let (val, left, right) = {
            let n = node.borrow();
            (n.val, n.left.clone(), n.right.clone())
        };

        if val == p || val == q {
            return Some(node);
        }

        let left = Self::dfs(left, p, q);
        let right = Self::dfs(right, p, q);
        match (left, right) {
            (Some(_), Some(_)) => Some(node),
            (found, None) | (None, found) => found,
        }
    }
}

// impl Solution {
//     pub fn lowest_common_ancestor(
//         root: Option<Rc<RefCell<TreeNode>>>,
//         p: Option<Rc<RefCell<TreeNode>>>,
//         q: Option<Rc<RefCell<TreeNode>>>,
//     ) -> Option<Rc<RefCell<TreeNode>>> {
//         // 題目保證所有 val 唯一，所以「比 val」== 「比是不是同一個節點」，
//         // 也避開 `curr == p` 那種會遞迴比整棵子樹的 O(子樹大小) 比較。
//         let p_val = p?.borrow().val;
//         let q_val = q?.borrow().val;
//         Self::dfs(root, p_val, q_val)
//     }

//     /// 回傳「這棵子樹裡，往上該回報的節點」：
//     /// - 子樹裡同時有 p 和 q -> 回報 LCA
//     /// - 只有其中一個       -> 回報那一個
//     /// - 都沒有             -> None
//     fn dfs(node: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
//         let node = node?; // 空子樹，沒東西可報

//         let (val, left, right) = {
//             let n = node.borrow(); // borrow 只活在這個 block，出去就還掉
//             (n.val, n.left.clone(), n.right.clone())
//         };

//         // 碰到 p 或 q 就直接往上報，不用再往下找另一個：
//         // 另一個若在我的子樹裡，答案本來就是我；
//         // 若不在，我這一支只會報一個上去，真正的分岔點在更上面。
//         if val == p || val == q {
//             return Some(node);
//         }

//         // 關鍵：資訊「由下往上」合併，而不是往下推計數器。
//         match (Self::dfs(left, p, q), Self::dfs(right, p, q)) {
//             (Some(_), Some(_)) => Some(node), // 左右各報上來一個 -> 我就是分岔點
//             (found, None) | (None, found) => found, // 只有一邊有（或都沒有）-> 原封不動往上傳
//         }
//     }
// }

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

    fn val(link: Link) -> Option<i32> {
        link.map(|n| n.borrow().val)
    }

    // 官方範例的樹 [3,5,1,6,2,0,8,null,null,7,4]
    // 回傳 (root, 節點5, 節點1, 節點4, 節點7, 節點2)
    fn example_tree() -> (Link, Link, Link, Link, Link, Link) {
        let n7 = leaf(7);
        let n4 = leaf(4);
        let n2 = node(2, n7.clone(), n4.clone());
        let n5 = node(5, leaf(6), n2.clone());
        let n1 = node(1, leaf(0), leaf(8));
        let root = node(3, n5.clone(), n1.clone());
        (root, n5, n1, n4, n7, n2)
    }

    // Example 1：p=5、q=1 分別在 root 的左右兩邊 -> LCA = 3
    // 抓「p、q 不在同一條 root-to-leaf 路徑上時，found 永遠湊不到 2」
    #[test]
    fn case1_example1() {
        let (root, n5, n1, _, _, _) = example_tree();
        assert_eq!(val(Solution::lowest_common_ancestor(root, n5, n1)), Some(3));
    }

    // Example 2：p=5 是 q=4 的祖先（隔兩層）-> LCA = 5（p 自己）
    // 抓「found 湊到 2 時回傳的是 parent，不是那個祖先本人」
    #[test]
    fn case2_example2() {
        let (root, n5, _, n4, _, _) = example_tree();
        assert_eq!(val(Solution::lowest_common_ancestor(root, n5, n4)), Some(5));
    }

    // 兩個兄弟 p=7、q=4 -> LCA = 2
    #[test]
    fn case3_siblings() {
        let (root, _, _, n4, n7, _) = example_tree();
        assert_eq!(val(Solution::lowest_common_ancestor(root, n7, n4)), Some(2));
    }

    // p 就是 root、q 在深處 -> LCA = root
    #[test]
    fn case4_p_is_root() {
        let (root, _, _, n4, _, _) = example_tree();
        assert_eq!(
            val(Solution::lowest_common_ancestor(root.clone(), root, n4)),
            Some(3)
        );
    }

    // 只有兩個節點 [1,2]，p=1、q=2 -> LCA = 1
    #[test]
    fn case5_two_nodes() {
        let n2 = leaf(2);
        let root = node(1, n2.clone(), None);
        assert_eq!(
            val(Solution::lowest_common_ancestor(root.clone(), root, n2)),
            Some(1)
        );
    }

    // p 是 q 的「直接父親」-> LCA = p
    // 這是目前寫法唯一會巧合答對的情況，留著當對照組
    #[test]
    fn case6_direct_parent() {
        let n3 = leaf(3);
        let n2 = node(2, n3.clone(), None);
        let root = node(1, n2.clone(), None);
        assert_eq!(val(Solution::lowest_common_ancestor(root, n2, n3)), Some(2));
    }

    // 左斜樹 1-2-3-4，p=2、q=4 隔了兩層 -> LCA = 2
    // 抓「回傳 q 的 parent（3）」這個 off-by-one
    #[test]
    fn case7_ancestor_two_levels() {
        let n4 = leaf(4);
        let n3 = node(3, n4.clone(), None);
        let n2 = node(2, n3, None);
        let root = node(1, n2.clone(), None);
        assert_eq!(val(Solution::lowest_common_ancestor(root, n2, n4)), Some(2));
    }
}
