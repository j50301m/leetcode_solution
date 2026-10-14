#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int singleNonDuplicate(vector<int> &nums) {
        int left = 0;
        int right = nums.size() - 1;

        while (left < right) {
            int mid = left + (right - left) / 2;

            if ((mid + 1) % 2 == 0) {
                if (nums[mid] == nums[mid - 1]) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            } else {
                if (nums[mid] == nums[mid - 1]) {
                    right = mid;
                } else {
                    left = mid;
                }
            }
        }

        return nums[left];
    }
};

void case1() {
    vector<int> vec{1, 1, 2, 2, 3};
    assert(Solution().singleNonDuplicate(vec) == 3);
}

// 答案在最前面
void case2() {
    vector<int> vec{1, 2, 2, 3, 3};
    assert(Solution().singleNonDuplicate(vec) == 1);
}

// 答案在中間
void case3() {
    vector<int> vec{1, 1, 2, 3, 3, 4, 4, 8, 8};
    assert(Solution().singleNonDuplicate(vec) == 2);
}

// 只有一個元素
void case4() {
    vector<int> vec{7};
    assert(Solution().singleNonDuplicate(vec) == 7);
}

// LeetCode 範例 2
void case5() {
    vector<int> vec{3, 3, 7, 7, 10, 11, 11};
    assert(Solution().singleNonDuplicate(vec) == 10);
}

// 長度 3，答案在最後
void case6() {
    vector<int> vec{1, 1, 2};
    assert(Solution().singleNonDuplicate(vec) == 2);
}

int main() {
    case1();
    case2();
    case3();
    case4();
    case5();
    case6();

    cout << "all pass" << endl;
}