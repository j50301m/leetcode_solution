#include <cassert>
#include <iostream>
#include <queue>
#include <unordered_set>
#include <vector>

using namespace std;
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x),
 * left(left), right(right) {}
 * };
 */
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right)
        : val(x), left(left), right(right) {}
};

struct Data {
    TreeNode *parent;
    TreeNode *curr;

    Data(TreeNode *parent, TreeNode *curr) : parent(parent), curr(curr) {}
};

class Solution {
public:
    TreeNode *correctBinaryTree(TreeNode *root) {
        queue<Data> que{};
        unordered_set<int> set{};
        que.push(Data(nullptr, root));
        while (!que.empty()) {
            Data data = que.front();
            if (data.curr->right != nullptr) {
                if (set.contains(data.curr->right->val)) {
                    if (data.parent->left == data.curr) {
                        data.parent->left = nullptr;
                    } else {
                        data.parent->right = nullptr;
                    }
                    return root;
                }
                que.push(Data(data.curr, data.curr->right));
            }
            if (data.curr->left != nullptr) {
                que.push(Data(data.curr, data.curr->left));
            }
            set.insert(data.curr->val);
            que.pop();
        }

        return root;
    }
};

int main() {
    // 範例 1：2 的 right 錯指到 3 → 移掉 2
    //      1
    //     / \
    //    2 → 3
    {
        TreeNode n1(1), n2(2), n3(3);
        n1.left = &n2, n1.right = &n3;
        n2.right = &n3; // 錯誤的邊

        TreeNode *root = Solution().correctBinaryTree(&n1);
        assert(root == &n1);
        assert(n1.left == nullptr);
        assert(n1.right == &n3);
    }

    // 範例 2：7 的 right 錯指到 4 → 移掉 7（連同底下的 2）
    //          8
    //        /   \
    //       3     1
    //      /     / \
    //     7  →  9   4
    //    /         / \
    //   2         5   6
    {
        TreeNode n8(8), n3(3), n1(1), n7(7), n9(9), n4(4), n2(2), n5(5), n6(6);
        n8.left = &n3, n8.right = &n1;
        n3.left = &n7;
        n1.left = &n9, n1.right = &n4;
        n7.left = &n2;
        n4.left = &n5, n4.right = &n6;
        n7.right = &n4; // 錯誤的邊

        TreeNode *root = Solution().correctBinaryTree(&n8);
        assert(root == &n8);
        assert(n3.left == nullptr);
        assert(n8.left == &n3 && n8.right == &n1);
        assert(n1.left == &n9 && n1.right == &n4);
        assert(n4.left == &n5 && n4.right == &n6);
    }

    // 錯誤節點是「右」子節點：5 的 right 錯指到 6 → 要清的是 2->right
    //          1
    //        /   \
    //       2     3
    //      / \   / \
    //     4   5→6   7
    {
        TreeNode n1(1), n2(2), n3(3), n4(4), n5(5), n6(6), n7(7);
        n1.left = &n2, n1.right = &n3;
        n2.left = &n4, n2.right = &n5;
        n3.left = &n6, n3.right = &n7;
        n5.right = &n6; // 錯誤的邊

        Solution().correctBinaryTree(&n1);
        assert(n2.right == nullptr);
        assert(n2.left == &n4);
        assert(n3.left == &n6 && n3.right == &n7);
    }

    // 錯指的目標不是緊鄰的節點：4 的 right 跳過 5、6 指到 7
    {
        TreeNode n1(1), n2(2), n3(3), n4(4), n5(5), n6(6), n7(7);
        n1.left = &n2, n1.right = &n3;
        n2.left = &n4, n2.right = &n5;
        n3.left = &n6, n3.right = &n7;
        n4.right = &n7; // 錯誤的邊

        Solution().correctBinaryTree(&n1);
        assert(n2.left == nullptr);
        assert(n2.right == &n5);
        assert(n3.left == &n6 && n3.right == &n7);
    }

    cout << "all pass" << endl;
}