#include <cassert>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> findClosestElements(vector<int> &arr, int k, int x) {
        int left = 0;
        int right = arr.size() - k;

        while (left < right) {
            int mid = left + (right - left) / 2;

            if (x - arr[mid] <= arr[mid + k] - x) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return vector<int>(arr.begin() + left, arr.begin() + left + k);
    }
};

void test_example_one() {
    vector<int> arr{1, 2, 3, 4, 5};
    assert((Solution().findClosestElements(arr, 4, 3) == vector<int>{1, 2, 3, 4}));
}

void test_x_smaller_than_all() {
    vector<int> arr{1, 1, 2, 3, 4, 5};
    assert((Solution().findClosestElements(arr, 4, -1) == vector<int>{1, 1, 2, 3}));
}

void test_x_larger_than_all() {
    vector<int> arr{1, 2, 3, 4, 5};
    assert((Solution().findClosestElements(arr, 2, 100) == vector<int>{4, 5}));
}

void test_not_consecutive() {
    vector<int> arr{1, 2, 5, 6, 10, 11};
    assert((Solution().findClosestElements(arr, 3, 7) == vector<int>{5, 6, 10}));
}

void test_tie_keeps_smaller() {
    vector<int> arr{1, 3, 5, 7};
    assert((Solution().findClosestElements(arr, 1, 4) == vector<int>{3}));
}

void test_duplicates() {
    vector<int> arr{1, 1, 2, 2, 2, 2, 2, 3, 3};
    assert((Solution().findClosestElements(arr, 3, 3) == vector<int>{2, 3, 3}));
}

void test_k_equals_size() {
    vector<int> arr{1, 2, 3};
    assert((Solution().findClosestElements(arr, 3, 2) == vector<int>{1, 2, 3}));
}

int main() {
    test_example_one();
    test_x_smaller_than_all();
    test_x_larger_than_all();
    test_not_consecutive();
    test_tie_keeps_smaller();
    test_duplicates();
    test_k_equals_size();
    cout << "all tests passed\n";
}