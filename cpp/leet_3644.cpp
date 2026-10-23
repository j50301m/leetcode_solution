#include <cassert>
#include <climits>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int sortPermutation(vector<int> &nums) {
        int result = -1;
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] != i) {
                result &= nums[i];
            }
        }

        if (result == -1) {
            return 0;
        }

        return result;
    }
};

void test_example_1() {
    vector<int> nums{0, 3, 2, 1};
    assert(Solution().sortPermutation(nums) == 1);
}

void test_example_2() {
    vector<int> nums{0, 1, 3, 2};
    assert(Solution().sortPermutation(nums) == 2);
}

void test_example_3() {
    vector<int> nums{3, 2, 1, 0};
    assert(Solution().sortPermutation(nums) == 0);
}

void test_already_sorted() {
    vector<int> nums{0, 1, 2, 3};
    assert(Solution().sortPermutation(nums) == 0);
}

void test_single() {
    vector<int> nums{0};
    assert(Solution().sortPermutation(nums) == 0);
}

int main() {
    test_example_1();
    test_example_2();
    test_example_3();
    test_already_sorted();
    test_single();
    cout << "all tests passed\n";
}