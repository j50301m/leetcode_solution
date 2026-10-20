#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    int triangleNumber(vector<int> &nums) {
        if (nums.size() < 3) {
            return 0;
        }
        // Desc order
        sort(nums.begin(), nums.end(), [](int a, int b) { return a > b; });

        int cnt = 0;
        for (int i = 0; i < nums.size() - 2; i++) {
            int long_side = nums[i];
            int boundary = nums.size();
            for (int j = i + 1; j < nums.size() - 1; j++) {
                if (boundary <= j + 1) {
                    break;
                }
                while (boundary > j + 1 &&
                       nums[j] + nums[boundary - 1] <= long_side) {
                    boundary -= 1;
                }
                cnt += boundary - (j + 1);
            }
        }

        return cnt;
    }
};

int run(vector<int> nums) { return Solution().triangleNumber(nums); }

int main() {
    assert(run({2, 2, 3, 4}) == 3);
    assert(run({4, 2, 3, 4}) == 4);
    assert(run({1}) == 0);
    assert(run({0, 0, 0}) == 0);
    assert(run({7, 6, 5, 4, 3, 2}) == 13);
    cout << "all pass" << endl;
}