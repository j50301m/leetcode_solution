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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(root) = root else {
            return None;
        };

        let val = root.borrow().val;
        if val > key {
            let child = root.borrow_mut().left.take();
            root.borrow_mut().left = Self::delete_node(child, key);
        } else if val < key {
            let child = root.borrow_mut().right.take();
            root.borrow_mut().right = Self::delete_node(child, key);
        } else {
            {
                let mut n = root.borrow_mut();
                if n.left.is_none() {
                    return n.right.take();
                }
                if n.right.is_none() {
                    return n.left.take();
                }
            }

            let right = root.borrow_mut().right.take();
            let mut curr = Rc::clone(right.as_ref().unwrap());
            let successor_val = loop {
                let next = curr.borrow().left.clone();
                if let Some(n) = next {
                    curr = n;
                } else {
                    break curr.borrow().val;
                }
            };

            root.borrow_mut().val = successor_val;

            root.borrow_mut().right = Self::delete_node(right, successor_val);
        }

        Some(root)
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

    // 測試用的樹，九個節點，把三種情況都湊齊了：
    //
    //         8          8  -> 雙子（successor 10 自己還有右小孩）
    //       /   \        3  -> 雙子（successor 4 是葉節點）
    //      3     10      10 -> 只有右小孩
    //     / \      \     14 -> 只有左小孩
    //    1   6      14   1 / 4 / 7 / 13 -> 葉節點
    //       / \     /
    //      4   7   13
    //
    // 中序：1 3 4 6 7 8 10 13 14
    fn example_tree() -> Link {
        node(
            8,
            node(3, leaf(1), node(6, leaf(4), leaf(7))),
            node(10, None, node(14, leaf(13), None)),
        )
    }

    fn inorder(root: &Link) -> Vec<i32> {
        fn go(n: &Link, out: &mut Vec<i32>) {
            if let Some(rc) = n {
                let b = rc.borrow();
                go(&b.left, out);
                out.push(b.val);
                go(&b.right, out);
            }
        }
        let mut out = Vec::new();
        go(root, &mut out);
        out
    }

    // 唯一需要的總檢查。
    //
    // 刻意不抓著舊節點的 Rc 來比對 —— delete_node 是「就地」修改，雙子節點那條路會
    // 直接覆蓋 node.val，所以測試前抓的 handle 值會跟著變，比對舊 handle 只會自己騙自己。
    //
    // 第二個 assert 看起來跟第一個重複，但它守的是「expected 這個字面陣列本身」：
    // 手寫八組預期序列，打錯一個數字的機率遠高於演算法出錯。
    fn assert_tree(root: &Link, expected: &[i32]) {
        let got = inorder(root);
        assert_eq!(got, expected, "中序走訪不符");
        assert!(
            got.windows(2).all(|w| w[0] < w[1]),
            "expected 自己就不是嚴格遞增，測資寫錯了：{:?}",
            expected
        );
    }

    // 情況 1：刪葉節點。父節點接住 None，節點就此消失。
    #[test]
    fn case1_delete_leaf() {
        let after = Solution::delete_node(example_tree(), 1);
        assert_tree(&after, &[3, 4, 6, 7, 8, 10, 13, 14]);
    }

    // 抓「搜尋方向寫反」：13 在右子樹最深處。
    // 如果 val > key 走了右邊，這裡會一路往左走到 None，13 不會被刪掉。
    #[test]
    fn case2_delete_leaf_in_right_subtree() {
        let after = Solution::delete_node(example_tree(), 13);
        assert_tree(&after, &[1, 3, 4, 6, 7, 8, 10, 14]);
    }

    // 情況 2 的左半邊：14 只有左小孩 13，13 要整棵頂上來。
    #[test]
    fn case3_only_left_child() {
        let after = Solution::delete_node(example_tree(), 14);
        assert_tree(&after, &[1, 3, 4, 6, 7, 8, 10, 13]);
    }

    // 情況 2 的右半邊：10 只有右小孩 14。
    // 只寫了 left.is_none() 而漏掉 right.is_none() 的話，這一題會直接漏抓 —— 10 不會被刪。
    #[test]
    fn case4_only_right_child() {
        let after = Solution::delete_node(example_tree(), 10);
        assert_tree(&after, &[1, 3, 4, 6, 7, 8, 13, 14]);
    }

    // 情況 3，successor 在深處：刪 3，successor = 4（右子樹一路往左走到底）。
    // 4 是葉節點ï¼所以第二次遞迴落在情況 1。
    #[test]
    fn case5_two_children() {
        let after = Solution::delete_node(example_tree(), 3);
        assert_tree(&after, &[1, 4, 6, 7, 8, 10, 13, 14]);
    }

    // 情況 3 而且是 root：successor = 10，走訪迴圈一次都不會跑（10 沒有左小孩）。
    // 10 自己有右小孩 14ï¼所以第二次遞迴落在情況 2 —— 跟 case5 互補的另一條路。
    // 這題也順便驗證「回傳值就是新的樹根」：接錯的話整棵樹會不見。
    #[test]
    fn case6_delete_root() {
        let after = Solution::delete_node(example_tree(), 8);
        assert_tree(&after, &[1, 3, 4, 6, 7, 10, 13, 14]);
    }

    // key 不存在：每一層都要原封不動把自己回傳回去。
    #[test]
    fn case7_key_not_found() {
        let after = Solution::delete_node(example_tree(), 99);
        assert_tree(&after, &[1, 3, 4, 6, 7, 8, 10, 13, 14]);
    }

    // 兩個退化輸入。
    #[test]
    fn case8_empty_and_single_node() {
        assert_eq!(inorder(&Solution::delete_node(None, 5)), Vec::<i32>::new());
        // 單節點樹刪掉自己 -> 整棵樹變空ï¼必須回 None 而不是那顆孤節點
        assert!(Solution::delete_node(leaf(5), 5).is_none());
        // 單節點樹刪別的 key -> 原封不動
        assert_tree(&Solution::delete_node(leaf(5), 7), &[5]);
    }

    // 最陰的那個 bug：take() 走的另一邊忘記放回去ï¼整棵子樹被 drop 掉。
    //
    // 這種錯誤用預期序列不一定抓得到ï¼因為「弄丟一整棵子樹之後ï¼剩下的序列還是嚴格遞增的」。
    // 露餡的是長度。逐一刪掉每個節點ï¼每次都必須剩下剛好 8 個。
    #[test]
    fn case9_never_lose_a_subtree() {
        for key in [1, 3, 4, 6, 7, 8, 10, 13, 14] {
            let got = inorder(&Solution::delete_node(example_tree(), key));
            assert_eq!(got.len(), 8, "刪 {} 之後節點數不對：{:?}", key, got);
            assert!(
                got.windows(2).all(|w| w[0] < w[1]),
                "刪 {} 之後不是嚴格遞增：{:?}",
                key,
                got
            );
            assert!(!got.contains(&key), "刪 {} 之後它還在：{:?}", key, got);
        }
    }
}
