#include <cassert>
#include <cstddef>
#include <iostream>

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
public:
    TreeNode *lowestCommonAncestor(TreeNode *root, TreeNode *p, TreeNode *q) {
        int p_val = p->val;
        int q_val = q->val;
        TreeNode *ans = nullptr;
        dfs(root, p_val, q_val, ans);
        return ans;
    }

    bool dfs(TreeNode *node, int p, int q, TreeNode *&found) {
        if (node == nullptr) {
            return false;
        }

        int curr = 0;
        if (node->val == p || node->val == q) {
            curr = 1;
        }

        int left = dfs(node->left, p, q, found) ? 1 : 0;
        int right = dfs(node->right, p, q, found) ? 1 : 0;

        if (left + right + curr == 2) {
            found = node;
        }

        if (left + right + curr >= 1) {
            return true;
        }

        return false;
    }
};

int main() {
    //         3
    //       /   \
    //      5     1
    //     / \   / \
    //    6   2 0   8
    //       / \
    //      7   4
    TreeNode n3(3), n5(5), n1(1), n6(6), n2(2), n0(0), n8(8), n7(7), n4(4);
    n3.left = &n5, n3.right = &n1;
    n5.left = &n6, n5.right = &n2;
    n1.left = &n0, n1.right = &n8;
    n2.left = &n7, n2.right = &n4;

    // 範例 1：分在兩邊 → 根
    assert(Solution().lowestCommonAncestor(&n3, &n5, &n1) == &n3);
    // 範例 2：p 是 q 的祖先 → p 自己
    assert(Solution().lowestCommonAncestor(&n3, &n5, &n4) == &n5);
    // 範例 3：q=10 不在樹裡 → nullptr（1644 跟 236 的差別）
    TreeNode missing(10);
    assert(Solution().lowestCommonAncestor(&n3, &n5, &missing) == nullptr);
    // 兩個都在深處同一棵子樹
    assert(Solution().lowestCommonAncestor(&n3, &n7, &n4) == &n2);
    assert(Solution().lowestCommonAncestor(&n3, &n6, &n4) == &n5);
    // q 是 p 的祖先（順序反過來）
    assert(Solution().lowestCommonAncestor(&n3, &n4, &n3) == &n3);
    // 只有一個節點，且 p 在樹裡、q 不在
    TreeNode single(1);
    assert(
        Solution().lowestCommonAncestor(&single, &single, &missing) == nullptr
    );

    std::cout << "all pass" << std::endl;
}