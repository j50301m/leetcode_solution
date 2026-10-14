#include <vector>

using namespace std;

class Solution {
public:
    int findKthPositive(vector<int> &arr, int k) {
        int left = 0;
        int right = arr.size();

        while (left < right) {
            int mid = left + (right - left) / 2;
            int miss_count = arr[mid] - mid;

            if (miss_count > k) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return left + k;
    }
};

int main() {}