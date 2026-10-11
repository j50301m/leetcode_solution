

/**
 * // This is the ArrayReader's API interface.
 * // You should not implement it, or speculate about its implementation
 * class ArrayReader {
 *   public:
 *     int get(int index);
 * };
 */
#include <cassert>
#include <cstdint>
#include <iostream>
#include <vector>

using namespace std;

class ArrayReader {
public:
    explicit ArrayReader(vector<int> secret) : secret(std::move(secret)) {}

    int get(int index) const {
        if (index < 0 || index >= (int)secret.size()) {
            return INT32_MAX;
        }
        return secret[index];
    }

private:
    vector<int> secret;
};

class Solution {
public:
    int search(const ArrayReader &reader, int target) {
        int left = 0;
        int right = INT32_MAX;

        while (left < right) {
            int mid = left + (right - left) / 2;

            if (reader.get(mid) >= target) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return reader.get(left) == target ? left : -1;
    }
};

void test_case1() {
    ArrayReader reader({-1, 0, 3, 5, 9, 12});
    assert(Solution().search(reader, 9) == 4);
}

void test_case2() {
    ArrayReader reader({-1, 0, 3, 5, 9, 12});
    assert(Solution().search(reader, 2) == -1);
}

void test_case3() {
    ArrayReader reader({5});
    assert(Solution().search(reader, 5) == 0);
}

void test_case4() {
    ArrayReader reader({-1, 0, 3, 5, 9, 12});
    assert(Solution().search(reader, -1) == 0);
    assert(Solution().search(reader, 12) == 5);
}

void test_case5() {
    ArrayReader reader({-1, 0, 3, 5, 9, 12});
    assert(Solution().search(reader, -5) == -1);
    assert(Solution().search(reader, 100) == -1);
}

int main() {
    test_case1();
    test_case2();
    test_case3();
    test_case4();
    test_case5();
    cout << "all tests passed\n";
}
