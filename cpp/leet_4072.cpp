#include <algorithm>
#include <cassert>
#include <climits>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    long long maxAlternatingSum(vector<int> &nums) {
        const long long MIN = LONG_MIN / 2;
        long long plus_0 = MIN;
        long long plus_1 = MIN;
        long long minus_0 = MIN;
        long long minus_1 = MIN;
        long long best = MIN;
        for (int num : nums) {
            long long temp_p0 = max((long long)num, minus_0 + num);
            long long temp_m0 = plus_0 - num;
            long long temp_p1 = max(minus_1 + num, plus_0);
            long long temp_m1 = max(plus_1 - num, minus_0);

            plus_0 = temp_p0;
            plus_1 = temp_p1;
            minus_0 = temp_m0;
            minus_1 = temp_m1;

            best = max({best, plus_0, plus_1, minus_0, minus_1});
        }
        return best;
    }
};

void test_case1() {
    vector<int> nums{5, -5, 1};
    assert(Solution().maxAlternatingSum(nums) == 11);
}

void test_case2() {
    vector<int> nums{10, -5, -100};
    assert(Solution().maxAlternatingSum(nums) == 110);
}

void test_case3() {
    vector<int> nums{94, 80, 20};
    assert(Solution().maxAlternatingSum(nums) == 94);
}

int main() {
    test_case1();
    test_case2();
    test_case3();
    cout << "all tests passed\n";
}
