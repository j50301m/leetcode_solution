#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int removeDuplicates(vector<int> &nums) {
        int appear_cnt = 1;
        int idx = 1;
        for (int i = 1; i < nums.size(); i++) {
            if (nums[i - 1] != nums[i]) {
                nums[idx] = nums[i];
                appear_cnt = 1;
                idx += 1;
                continue;
            }

            appear_cnt += 1;
            if (appear_cnt <= 2) {
                nums[idx] = nums[i];
                idx += 1;
            }
        }

        return idx;
    }
};

// 檢查回傳的 k，以及 nums 前 k 個元素
void check(vector<int> nums, vector<int> expected) {
    int k = Solution().removeDuplicates(nums);
    assert(k == expected.size());
    assert(vector<int>(nums.begin(), nums.begin() + k) == expected);
}

void test_example1() { check({1, 1, 1, 2, 2, 3}, {1, 1, 2, 2, 3}); }

void test_example2() {
    check({0, 0, 1, 1, 1, 1, 2, 3, 3}, {0, 0, 1, 1, 2, 3, 3});
}

void test_single() { check({1}, {1}); }

void test_all_same() { check({2, 2, 2, 2}, {2, 2}); }

void test_no_duplicates() { check({1, 2, 3}, {1, 2, 3}); }

int main() {
    test_example1();
    test_example2();
    test_single();
    test_all_same();
    test_no_duplicates();
    cout << "all tests passed\n";
}