#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int peakIndexInMountainArray(vector<int> &arr) {
        int left = 0;
        int right = arr.size() - 1;

        while (left < right) {
            int mid = left + (right - left) / 2;

            if (arr[mid] <= arr[mid + 1]) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        return left;
    }
};

void test_peak_in_middle() {
    vector<int> arr{0, 1, 0};
    assert(Solution().peakIndexInMountainArray(arr) == 1);
}

void test_peak_left_side() {
    vector<int> arr{0, 10, 5, 2};
    assert(Solution().peakIndexInMountainArray(arr) == 1);
}

void test_peak_right_side() {
    vector<int> arr{0, 2, 5, 10, 1};
    assert(Solution().peakIndexInMountainArray(arr) == 3);
}

void test_long_mountain() {
    vector<int> arr{3, 5, 7, 9, 11, 13, 12, 8, 4, 1};
    assert(Solution().peakIndexInMountainArray(arr) == 5);
}

int main() {
    test_peak_in_middle();
    test_peak_left_side();
    test_peak_right_side();
    test_long_mountain();
    cout << "all tests passed\n";
}