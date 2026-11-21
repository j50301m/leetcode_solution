#include <vector>

using namespace std;

class Solution {
public:
    int minEatingSpeed(vector<int> &piles, int h) {
        sort(piles.begin(), piles.end());
        int n = piles.size();
        int left = 1;
        int right = piles[n - 1];

        while (left < right) {
            int mid = left + (right - left) / 2;

            int spend_h = 0;
            for (auto num : piles) {
                int hour = num % mid == 0 ? num / mid : num / mid + 1;
                spend_h += hour;
            }

            if (spend_h > h) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        return left;
    }
};

int main() {}