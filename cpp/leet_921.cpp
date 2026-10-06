#include <cassert>
#include <iostream>
#include <stack>
#include <string>

using namespace std;

class Solution {
public:
    int minAddToMakeValid(string s) {
        stack<char> stack{};
        int acc = 0;
        for (char c : s) {
            if (c == '(') {
                stack.push('(');
                continue;
            }

            if (stack.empty()) {
                acc += 1;
                continue;
            }
            stack.pop();
        }

        return stack.size() + acc;
    }
};

void case1() { assert(Solution().minAddToMakeValid("())") == 1); }

int main() {
    case1();
    cout << "all pass" << endl;
}