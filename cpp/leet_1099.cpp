#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

using namespace std;
class Solution {
public:
    int twoSumLessThanK(vector<int> &nums, int k) {
        sort(nums.begin(), nums.end());
        int best = -1;
        for (int i = 0; i < nums.size() - 1; i++) {
            int needed = k - nums[i];
            if (needed <= nums[i]) {
                break;
            }
            auto lo = nums.begin() + i + 1;
            auto pos = partition_point(lo, nums.end(), [&](int x) {
                return x < needed;
            });
            if (pos == lo) {
                continue;
            }
            best = max(best, nums[i] + *(pos - 1));
        }

        return best;
    }
};

void case1() {
    vector<int> nums = {34, 23, 1, 24, 75, 33, 54, 8};
    assert(Solution().twoSumLessThanK(nums, 60) == 58);
}

void case2() {
    vector<int> nums = {10, 20, 30};
    assert(Solution().twoSumLessThanK(nums, 15) == -1);
}

void case3() {
    vector<int> nums = {5};
    assert(Solution().twoSumLessThanK(nums, 100) == -1);
}

void case4() {
    vector<int> nums = {5, 5, 5};
    assert(Solution().twoSumLessThanK(nums, 11) == 10);
}

void case5() {
    vector<int> nums = {1, 2, 3};
    assert(Solution().twoSumLessThanK(nums, 100) == 5);
}

int main() {
    case1();
    case2();
    case3();
    case4();
    case5();

    cout << "all pass" << endl;
}