#include <cassert>
#include <iostream>
#include <string>

using namespace std;

class Solution {
public:
    string maxSumOfSquares(int num, int sum) {
        string ans{};
        for (int i = 0; i < num; i++) {
            if (sum < 9) {
                char x = sum + '0';
                ans.push_back(x);
                sum = 0;
            } else {
                sum -= 9;
                ans.push_back('9');
            }
        }
        if (sum > 0) {
            return "";
        }
        return ans;
    }
};

void case1() { assert(Solution().maxSumOfSquares(2, 3) == "30"); }

void case2() { assert(Solution().maxSumOfSquares(2, 17) == "98"); }

void case3() { assert(Solution().maxSumOfSquares(1, 10) == ""); }

int main() {
    case1();
    case2();
    case3();

    cout << "all pass" << endl;
}