#include <cassert>
#include <iostream>
#include <algorithm>
#include <stack>
#include <vector>

using namespace std;

class Solution {
public:
    int findMin(vector<int> &nums) {
        stack<int> st{};
        int last_ele = nums.back();
        int pos = partition_point(
                      nums.begin(),
                      nums.end(),
                      [&](int x) { return x > last_ele; }
                  ) -
                  nums.begin();
        return nums[pos];
    }
};

void test_rotated() {
    vector<int> nums{3, 4, 5, 1, 2};
    assert(Solution().findMin(nums) == 1);
}

void test_not_rotated() {
    vector<int> nums{1, 2, 3};
    assert(Solution().findMin(nums) == 1);
}

void test_two() {
    vector<int> nums{2, 1};
    assert(Solution().findMin(nums) == 1);
}

void test_single() {
    vector<int> nums{7};
    assert(Solution().findMin(nums) == 7);
}

int main() {
    test_rotated();
    test_not_rotated();
    test_two();
    test_single();
    cout << "all tests passed\n";
}
