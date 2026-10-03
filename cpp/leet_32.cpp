#include <cassert>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int longestValidParentheses(string s) {
        vector<int> stack{-1};
        int max_len = 0;
        for (int i = 0; i < s.size(); i++) {
            char c = s[i];
            if (c == '(') {
                stack.push_back(i);
                continue;
            }
            stack.pop_back();
            if (!stack.empty()) {
                max_len = max(max_len, i - stack[stack.size() - 1]);
                continue;
            }
            stack.push_back(i);
        }

        return max_len;
    }
};

void test_examples() {
    assert(Solution().longestValidParentheses("(()") == 2);
    assert(Solution().longestValidParentheses(")()())") == 4);
    assert(Solution().longestValidParentheses("") == 0);
}

void test_no_valid() {
    assert(Solution().longestValidParentheses("(((") == 0);
    assert(Solution().longestValidParentheses(")))") == 0);
    assert(Solution().longestValidParentheses(")(") == 0);
}

void test_split_by_unmatched() {
    assert(Solution().longestValidParentheses("(()(()") == 2);
    assert(Solution().longestValidParentheses("()(()") == 2);
}

void test_concat_after_nested() {
    assert(Solution().longestValidParentheses("()(())") == 6);
    assert(Solution().longestValidParentheses("(()())") == 6);
}

int main() {
    test_examples();
    test_no_valid();
    test_split_by_unmatched();
    test_concat_after_nested();
    cout << "all tests passed\n";
}