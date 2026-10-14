#include <algorithm>
#include <functional>
#include <numeric>
#include <vector>

using namespace std;

class Solution {
public:
    long long maxAlternatingSum(vector<int> &nums) {
        int mid = nums.size() / 2;

        nth_element(
            nums.begin(),
            nums.begin() + mid,
            nums.end(),
            [](int left, int right) { return abs(left) < abs(right); }
        );

        long long positive_sum = transform_reduce(
            nums.begin() + mid,
            nums.end(),
            0LL,
            plus<>{},
            [](int value) { return 1LL * value * value; }
        );

        long long negative_sum = transform_reduce(
            nums.begin(),
            nums.begin() + mid,
            0LL,
            plus<>{},
            [](int value) { return 1LL * value * value; }
        );

        return positive_sum - negative_sum;
    }
};

int main() {}