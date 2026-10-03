#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int search(vector<int> &nums, int target) {
        if (nums.empty()) {
            return -1;
        }

        int left = 0;
        int right = nums.size() - 1;

        while (left <= right) {
            int mid = left + (right - left) / 2;
            int found = nums[mid];
            if (found == target)
                return mid;
            else if (found > target)
                right = mid - 1;
            else if (found < target)
                left = mid + 1;
        }

        return -1;
    }

    int search_with_api(vector<int> &nums, int target) {
        auto it = lower_bound(nums.begin(), nums.end(), target);
        if (it != nums.end() && *it == target) {
            return it - nums.begin();
        }

        return -1;
    }
};

void test_found() {
    vector<int> nums{-1, 0, 3, 5, 9, 12};
    assert(Solution().search(nums, 9) == 4);
    assert(Solution().search_with_api(nums, 9) == 4);
}

void test_not_found() {
    vector<int> nums{-1, 0, 3, 5, 9, 12};
    assert(Solution().search(nums, 2) == -1);
}

void test_single() {
    vector<int> nums{5};
    assert(Solution().search(nums, 5) == 0);
    assert(Solution().search(nums, -5) == -1);
}

int main() {
    test_found();
    test_not_found();
    test_single();
    cout << "all tests passed\n";
}