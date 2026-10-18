#include <vector>
using namespace std;

class NumArray {
    int n;
    vector<int> tree{};

public:
    NumArray(vector<int> &nums) {
        n = nums.size();

        // Build lead node
        for (int i = 0; i < n; i++) {
            tree[n + i] = nums[i];
        }
    }

    int sumRange(int left, int right) {}
};

int main() {}