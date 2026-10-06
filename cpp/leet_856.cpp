#include <cassert>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int scoreOfParentheses(string s) {
        vector<int> stack{};
        stack.push_back(0);

        for (char c : s) {
            if (c == '(') {
                stack.push_back(0);
                continue;
            }

            int score = stack.back();
            stack.pop_back();
            if (score == 0) {
                score = 1;
            } else {
                score *= 2;
            }

            stack.back() += score;
        }

        return stack[0];
    }
};

int run(string s) { return Solution().scoreOfParentheses(s); }

void test_examples() {
    assert(run("()") == 1);
    assert(run("(())") == 2);
    assert(run("()()") == 2);
}

void test_siblings_inside_nest() { assert(run("(()(()))") == 6); }

void test_deep_nest() { assert(run("((()))") == 4); }

void test_mixed() { assert(run("(()())(())") == 6); }

int main() {
    test_examples();
    test_siblings_inside_nest();
    test_deep_nest();
    test_mixed();
    cout << "all tests passed\n";
}
